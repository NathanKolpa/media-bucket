use std::net::IpAddr;
use serde::{Serialize, Deserialize};

use chrono::{DateTime, Duration, TimeZone, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

const KEY_ALGO: Algorithm = Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: i64,
    ip: String,
    exp: i64,
}

pub struct AuthToken {
    ip: IpAddr,
    issued_at: DateTime<Utc>,
    lifetime: Duration,
}

impl AuthToken {
    pub fn new(ip: IpAddr, issued_at: DateTime<Utc>, lifetime: Duration) -> Self {
        Self {
            lifetime,
            ip,
            issued_at,
        }
    }

    pub fn to_token(&self, secret: &[u8]) -> String {
        let key = EncodingKey::from_secret(secret);
        let headers = Header::new(KEY_ALGO);


        let claims = Claims {
            ip: self.ip.to_string(),
            iat: self.issued_at.timestamp(),
            exp: self.expires_at(),
        };

        jsonwebtoken::encode(&headers, &claims, &key).unwrap()
    }

    pub fn expires_at(&self) -> i64 {
        self.issued_at.timestamp() + self.lifetime.num_seconds()
    }

    pub fn from_token(
        token: &str,
        secret: &[u8],
        ip: &IpAddr,
    ) -> Option<Self> {
        let key = DecodingKey::from_secret(secret);
        let ip_str = ip.to_string();

        let claims = jsonwebtoken::decode::<Claims>(token, &key, &Validation::new(KEY_ALGO)).ok()?;

        let iat = Utc.timestamp_opt(claims.claims.iat, 0)
            .unwrap();

        let offset = claims.claims.exp - claims.claims.iat;

        if claims.claims.ip != ip_str {
            return None;
        }

        Some(Self {
            ip: claims.claims.ip.parse().ok()?,
            lifetime: Duration::seconds(offset),
            issued_at: iat,
        })
    }
}
