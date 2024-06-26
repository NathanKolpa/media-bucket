use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Duration, Utc};
use rand::{thread_rng, Rng};
use thiserror::Error;
use url::Url;

use crate::data_source::DataSourceError;
use crate::http_server::instance::LoginError::LoadingError;
use crate::http_server::token::AuthToken;
use crate::{Bucket, BucketError};

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Invalid password")]
    InvalidPassword,

    #[error("Password is required")]
    PasswordRequired,

    #[error("Loading error")]
    LoadingError(BucketError),

    #[error("Fetching error")]
    FetchingError(#[from] DataSourceError),
}

impl From<BucketError> for LoginError {
    fn from(value: BucketError) -> Self {
        match value {
            BucketError::PasswordRequired => Self::PasswordRequired,
            BucketError::InvalidPassword => Self::InvalidPassword,
            e => LoadingError(e),
        }
    }
}

pub struct Session {
    parent: Arc<ServerBucketInstance>,
    bucket: Arc<Bucket>,
    ip: IpAddr,
    read_only: bool,
    token: Option<String>,
}

impl Session {
    pub fn bucket(&self) -> &Bucket {
        self.bucket.deref()
    }

    pub fn bucket_arc(&self) -> Arc<Bucket> {
        self.bucket.clone()
    }

    pub fn instance(&self) -> &ServerBucketInstance {
        self.parent.deref()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn get_session_count() {}
}

pub struct ServerBucketInstance {
    id: u64,
    location: String,
    name: String,
    base_url: Option<Arc<Url>>,
    password_protected: bool,
    instance: RwLock<Option<Arc<Bucket>>>,
    sessions_created: AtomicU64,
    token_secret: RwLock<Option<[u8; 32]>>,
    hidden: bool,
    randomize_secret: bool,
    last_login: AtomicU64,
    session_lifetime: Duration,
}

pub struct NewLogin {
    pub token: String,
    pub share_token: String,
    pub lifetime: Duration,
    pub now: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl ServerBucketInstance {
    pub async fn load(
        id: u64,
        name: String,
        base_url: Option<Arc<Url>>,
        location: String,
        hidden: bool,
        randomize_secret: bool,
        session_lifetime: Duration,
    ) -> std::io::Result<Self> {
        Ok(ServerBucketInstance {
            id,
            password_protected: Bucket::password_protected(location.as_str()).await?,
            location,
            name,
            instance: Default::default(),
            sessions_created: AtomicU64::new(0),
            base_url,
            token_secret: Default::default(),
            hidden,
            randomize_secret,
            last_login: AtomicU64::new(0),
            session_lifetime,
        })
    }

    pub fn password_protected(&self) -> bool {
        self.password_protected
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn base_url(&self) -> Option<Arc<Url>> {
        self.base_url.clone()
    }

    pub fn authorize_token(self: Arc<Self>, token: String, ip: IpAddr) -> Option<Session> {
        let token_secret = (*self.token_secret.read().unwrap())?;

        let auth_token = AuthToken::from_token(&token, &token_secret, &ip)?;

        let bucket = self.instance.read().unwrap().clone()?;

        Some(Session {
            token: Some(token),
            bucket,
            parent: self,
            ip,
            read_only: auth_token.read_only(),
        })
    }
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn last_login(&self) -> Option<DateTime<Utc>> {
        let timestamp = self.last_login.load(Ordering::Relaxed);

        if timestamp == 0 {
            return None;
        }

        DateTime::from_timestamp(timestamp as i64, 0)
    }

    pub fn unload(&self) {
        let mut instance = self.instance.write().unwrap();
        let mut secret = self.token_secret.write().unwrap();

        self.last_login.store(0, Ordering::Release);
        self.sessions_created.store(0, Ordering::Release);

        *instance = None;
        *secret = None;
    }

    pub fn should_unload_at(&self) -> Option<DateTime<Utc>> {
        self.last_login().map(|date| date + self.session_lifetime)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn login(&self, password: Option<&str>, ip: IpAddr) -> Result<NewLogin, LoginError> {
        let instance = self.instance.read().unwrap();

        let token_secret;

        if let Some(bucket) = &*instance {
            // the instance is loaded
            if !self.randomize_secret {
                token_secret = bucket
                    .data_source()
                    .passwords()
                    .validate_password(password)
                    .await?;
            } else {
                token_secret = *self.token_secret.read().unwrap();
            }
        } else {
            // load the instance
            drop(instance);

            let bucket = Bucket::open(self.location.as_str(), password).await?;
            bucket.data_source().cross().gc().await?;

            if !self.randomize_secret {
                token_secret = bucket
                    .data_source()
                    .passwords()
                    .validate_password(password)
                    .await?;
            } else {
                token_secret = Some(thread_rng().gen());
            }

            let mut instance = self.instance.write().unwrap();
            *instance = Some(Arc::new(bucket));

            let mut instance_secret = self.token_secret.write().unwrap();
            *instance_secret = token_secret;
        }

        let Some(token_secret) = token_secret else {
            return Err(LoginError::InvalidPassword);
        };

        let now = Utc::now();

        let new_token = self.new_session(ip, false, now).to_token(&token_secret);
        let ro_token = self.new_session(ip, true, now).to_token(&token_secret);

        let last_login = self.last_login();
        self.update_last_login(now);

        Ok(NewLogin {
            now,
            lifetime: self.session_lifetime,
            token: new_token,
            share_token: ro_token,
            last_login,
        })
    }

    fn update_last_login(&self, now: DateTime<Utc>) {
        let unix_now = now.timestamp() as u64;
        let mut current_last_login = self.last_login.load(Ordering::Relaxed);
        while unix_now > current_last_login {
            match self.last_login.compare_exchange_weak(
                current_last_login,
                unix_now,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(c) => current_last_login = c,
            }
        }
    }

    fn new_session(&self, ip: IpAddr, read_only: bool, now: DateTime<Utc>) -> AuthToken {
        let token = AuthToken::new(ip, now, self.session_lifetime, read_only);

        self.sessions_created.fetch_add(1, Ordering::Relaxed);

        token
    }

    pub fn sessions_created(&self) -> u64 {
        self.sessions_created.load(Ordering::Relaxed)
    }

    pub fn is_bfu(&self) -> bool {
        self.token_secret.read().unwrap().is_none() && !self.randomize_secret
    }
}

impl Display for ServerBucketInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" ({})", self.name, self.id)
    }
}

#[derive(Default)]
pub struct InstanceDataSource {
    instances: HashMap<u64, Arc<ServerBucketInstance>>,
}

impl InstanceDataSource {
    pub fn get_by_id(&self, id: u64) -> Option<Arc<ServerBucketInstance>> {
        self.instances.get(&id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<ServerBucketInstance>> {
        self.instances.values().cloned().collect()
    }

    pub fn all_sorted(&self) -> Vec<Arc<ServerBucketInstance>> {
        let mut list = self.all();

        list.sort_by(|a, b| a.id.cmp(&b.id));

        list
    }

    pub fn new(instances: Vec<Arc<ServerBucketInstance>>) -> Self {
        let mut map = HashMap::with_capacity(instances.len());
        for instance in instances {
            map.insert(instance.id, instance);
        }

        Self { instances: map }
    }
}
