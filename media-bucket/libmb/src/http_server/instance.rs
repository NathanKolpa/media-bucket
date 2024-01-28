use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use chrono::{Duration, Utc};
use thiserror::Error;

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
}

impl Session {
    pub fn bucket(&self) -> &Bucket {
        self.bucket.deref()
    }

    pub fn instance(&self) -> &ServerBucketInstance {
        self.parent.deref()
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
    password_protected: bool,
    instance: RwLock<Option<Arc<Bucket>>>,
    sessions_created: AtomicU64,
    token_secret: RwLock<Option<[u8; 32]>>,
}

impl ServerBucketInstance {
    pub async fn load(id: u64, name: String, location: String) -> std::io::Result<Self> {
        Ok(ServerBucketInstance {
            id,
            password_protected: Bucket::password_protected(location.as_str()).await?,
            location,
            name,
            instance: Default::default(),
            sessions_created: AtomicU64::new(0),
            token_secret: Default::default(),
        })
    }

    pub fn password_protected(&self) -> bool {
        self.password_protected
    }

    pub fn authorize_token(self: Arc<Self>, token: &str, ip: IpAddr) -> Option<Session> {
        let token_secret = self.token_secret.read().unwrap().clone()?;

        let auth_token = AuthToken::from_token(token, &token_secret, &ip)?;

        let bucket = self.instance.read().unwrap().clone()?;

        Some(Session {
            bucket,
            parent: self,
            ip,
            read_only: auth_token.read_only(),
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn login(&self, password: Option<&str>, ip: IpAddr) -> Result<(String, String), LoginError> {
        let instance = self.instance.read().unwrap();

        let token_secret;

        if let Some(bucket) = &*instance {
            // the instance is loaded
            token_secret = bucket
                .data_source()
                .passwords()
                .validate_password(password)
                .await?;
        } else {
            // load the instance
            drop(instance);

            let bucket = Bucket::open(self.location.as_str(), password).await?;

            token_secret = bucket
                .data_source()
                .passwords()
                .validate_password(password)
                .await?;

            let mut instance = self.instance.write().unwrap();
            *instance = Some(Arc::new(bucket));

            let mut instance_secret = self.token_secret.write().unwrap();
            *instance_secret = token_secret;
        }

        let Some(token_secret) = token_secret else {
            return Err(LoginError::InvalidPassword);
        };

        let new_token = self.new_session(ip, false).to_token(&token_secret);
        let ro_token = self.new_session(ip, true).to_token(&token_secret);


        Ok((new_token, ro_token))
    }

    fn new_session(&self, ip: IpAddr, read_only: bool) -> AuthToken {
        let now = Utc::now();
        let lifetime = Duration::days(14);

        let token = AuthToken::new(ip, now, lifetime, read_only);

        self.sessions_created.fetch_add(1, Ordering::Relaxed);

        token
    }

    pub fn sessions_created(&self) -> u64 {
        self.sessions_created.load(Ordering::Relaxed)
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
