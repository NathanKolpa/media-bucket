use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use thiserror::Error;

use crate::data_source::DataSourceError;
use crate::http_server::instance::LoginError::LoadingError;
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

#[derive(Clone)]
struct SessionData {
    created_at: DateTime<Utc>,
    ip: IpAddr,
}

pub struct Session {
    token: String,
    parent: Arc<ServerBucketInstance>,
    bucket: Arc<Bucket>,
}

impl Session {
    pub fn bucket(&self) -> &Bucket {
        self.bucket.deref()
    }

    pub fn instance(&self) -> &ServerBucketInstance {
        self.parent.deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.parent
            .get_session_data(self.token.as_str())
            .unwrap()
            .created_at
    }

    pub fn ip(&self) -> IpAddr {
        self.parent
            .get_session_data(self.token.as_str())
            .unwrap()
            .ip
    }

    pub fn get_session_count() {}

    pub fn logout(&self) {
        self.parent.logout(self.token.as_str())
    }
}

pub struct ServerBucketInstance {
    id: u64,
    location: String,
    name: String,
    password_protected: bool,
    instance: RwLock<Option<Arc<Bucket>>>,
    sessions: RwLock<HashMap<String, SessionData>>,
    last_activity: RwLock<Instant>,
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
            last_activity: RwLock::new(Instant::now()),
        })
    }

    fn get_session_data(&self, token: &str) -> Option<SessionData> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(token).cloned()
    }

    pub fn password_protected(&self) -> bool {
        self.password_protected
    }

    pub fn get_session_by_token(self: Arc<Self>, token: String) -> Option<Session> {
        let valid_token = self.sessions.read().unwrap().contains_key(token.as_str());
        let bucket = self.instance.read().unwrap().clone();

        if let Some(bucket) = bucket {
            if valid_token {
                Some(Session {
                    token,
                    bucket,
                    parent: self,
                })
            } else {
                None
            }
        } else {
            None
        }
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

        let new_token: String = Self::random_token();

        let session_data = SessionData {
            ip,
            created_at: Utc::now(),
        };

        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(new_token.clone(), session_data);

        Ok(new_token)
    }

    pub fn logout(&self, token: &str) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(token);
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
