use std::{fmt::Display, sync::Arc};

use url::Url;

pub struct ApiUrl {
    pub bucket_id: u64,
    pub base: Option<Arc<Url>>,
}

impl Display for ApiUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_url_str = self.base.as_ref().map(|x| x.as_str()).unwrap_or("");
        write!(f, "{}buckets/{}", base_url_str, self.bucket_id)
    }
}

pub struct AuthParams {
    pub token: Option<String>,
}

impl AuthParams {
    pub fn without_include(&self) -> impl Display + '_ {
        AuthParamsDisplay {
            params: self,
            forward: false,
        }
    }

    pub fn include_token(&self) -> impl Display + '_ {
        AuthParamsDisplay {
            params: self,
            forward: true,
        }
    }
}

struct AuthParamsDisplay<'a> {
    params: &'a AuthParams,
    forward: bool,
}

impl Display for AuthParamsDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str = self
            .params
            .token
            .as_ref()
            .map(|t| format!("?token={}", t))
            .unwrap_or_default();

        let include_str = if self.forward && self.params.token.is_some() {
            "&include_token=true"
        } else {
            ""
        };

        write!(f, "{token_str}{include_str}")
    }
}
