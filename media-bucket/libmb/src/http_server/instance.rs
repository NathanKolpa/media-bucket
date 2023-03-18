use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicI64, Ordering};

use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use thiserror::Error;

use crate::data_source::DataSourceError;
use crate::http_server::instance::LoginError::LoadingError;
use crate::{Bucket, BucketError};
use crate::http_server::token::AuthToken;

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

#[derive(Clone)]
struct SessionData {
    created_at: DateTime<Utc>,
    ip: IpAddr,
}

pub struct Session {
    id: i64,
    parent: Arc<ServerBucketInstance>,
    bucket: Arc<Bucket>,
    ip: IpAddr
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

    pub fn get_session_count() {}

    pub fn logout(&self) {
        self.parent.logout(self.id)
    }
}

pub struct ServerBucketInstance {
    id: u64,
    location: String,
    name: String,
    password_protected: bool,
    instance: RwLock<Option<Arc<Bucket>>>,
    sessions: DashMap<i64, SessionData>,
    session_ai: AtomicI64,
    token_secret: [u8; 32]
}

impl ServerBucketInstance {
    pub async fn load(id: u64, name: String, location: String) -> std::io::Result<Self> {
        Ok(ServerBucketInstance {
            id,
            password_protected: Bucket::password_protected(location.as_str()).await?,
            location,
            name,
            instance: Default::default(),
            sessions: Default::default(),
            token_secret: thread_rng().gen(),
            session_ai: Default::default()
        })
    }

    pub fn password_protected(&self) -> bool {
        self.password_protected
    }

    pub fn authorize_token(self: Arc<Self>, token: &str, ip: IpAddr) -> Option<Session> {
        let auth_token = AuthToken::from_token(token, &self.token_secret, &ip)?;
        let bucket = self.instance.read().unwrap().clone()?;

        Some(Session {
            id: auth_token.session_id(),
            bucket,
            parent: self,
            ip
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn login(&self, password: Option<&str>, ip: IpAddr) -> Result<String, LoginError> {
        {
            let instance = self.instance.read().unwrap();

            if let Some(bucket) = &*instance {
                // the instance is loaded
                if !bucket
                    .data_source()
                    .passwords()
                    .is_valid_password(password)
                    .await?
                {
                    return Err(LoginError::InvalidPassword);
                }
            } else {
                // load the instance
                drop(instance);

                let bucket = Bucket::open(self.location.as_str(), password).await?;

                let mut instance = self.instance.write().unwrap();
                *instance = Some(Arc::new(bucket));
            }
        }

        let new_token = self.new_session(ip).to_token(&self.token_secret);
        Ok(new_token)
    }

    fn new_session(&self, ip: IpAddr) -> AuthToken {
        let session_id = self.session_ai.fetch_add(1, Ordering::SeqCst);
        let now = Utc::now();
        let lifetime = Duration::days(3);

        let token = AuthToken::new(session_id, ip, now.clone(), lifetime);

        let session_data = SessionData {
            ip,
            created_at: now,
        };

        self.sessions.insert(session_id, session_data);

        token
    }

    pub fn logout(&self, session_id: i64) {
        self.sessions.remove(&session_id);
    }

    fn random_token() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect()
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
