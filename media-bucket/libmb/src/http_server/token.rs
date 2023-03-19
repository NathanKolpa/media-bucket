use std::net::IpAddr;
use serde::{Serialize, Deserialize};

use chrono::{DateTime, Duration, TimeZone, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

const KEY_ALGO: Algorithm = Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: i64,
    iss: String,
    exp: i64,
    session_id: i64,
}

pub struct AuthToken {
    session_id: i64,
    ip: IpAddr,
    issued_at: DateTime<Utc>,
    lifetime: Duration,
}

impl AuthToken {
    pub fn new(session_id: i64, ip: IpAddr, issued_at: DateTime<Utc>, lifetime: Duration) -> Self {
        Self {
            lifetime,
            session_id,
            ip,
            issued_at,
        }
    }

    pub fn to_token(&self, secret: &[u8]) -> String {
        let key = EncodingKey::from_secret(secret);
        let headers = Header::new(KEY_ALGO);


        let claims = Claims {
            iss: self.ip.to_string(),
            iat: self.issued_at.timestamp(),
            exp: self.expires_at(),
            session_id: self.session_id,
        };

        jsonwebtoken::encode(&headers, &claims, &key).unwrap()
    }

    pub fn expires_at(&self) -> i64 {
        self.issued_at.timestamp() + self.lifetime.num_seconds()
    }

    pub fn session_id(&self) -> i64 {
        self.session_id
    }

    pub fn from_token(
        token: &str,
        secret: &[u8],
        ip: &IpAddr,
    ) -> Option<Self> {
        let key = DecodingKey::from_secret(secret);
        let mut validation = Validation::new(KEY_ALGO);
        let ip_str = ip.to_string();
        validation.set_issuer(&[ip_str]);

        let claims = jsonwebtoken::decode::<Claims>(token, &key, &validation).ok()?;

        let iat = Utc.timestamp_opt(claims.claims.iat, 0)
            .unwrap();

        let offset = claims.claims.exp - claims.claims.iat;

        Some(Self {
            ip: claims.claims.iss.parse().ok()?,
            lifetime: Duration::seconds(offset),
            issued_at: iat,
            session_id: claims.claims.session_id
        })
    }
}