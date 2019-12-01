use std::str::FromStr;

use chrono::{Duration, Utc};
use hyper::{
    client::HttpConnector,
    header::{self, HeaderValue, ToStrError},
    Body, Client, Request, Uri,
};
use hyper_rustls::HttpsConnector;
use jsonwebtoken;
use serde::Serialize;
use snafu::{ResultExt, Snafu};

pub struct LucidClient {
    client: Client<HttpsConnector<HttpConnector>>,
    uri: Uri,
    secret: Option<String>,
}

impl LucidClient {
    pub fn new(uri: &str, secret: Option<String>) -> Result<Self, LucidClientError> {
        if let Ok(uri) = Uri::from_str(uri) {
            if uri.scheme_part().is_none() || uri.authority_part().is_none() {
                Err(LucidClientError::InvalidUri)
            } else {
                Ok(Self {
                    client: Client::builder().build({ HttpsConnector::new() }),
                    uri: uri,
                    secret: secret,
                })
            }
        } else {
            Err(LucidClientError::InvalidUri)
        }
    }

    pub async fn get(&self, key: &str) -> Result<Option<KvGetResponse>, LucidClientError> {
        let mut builder = Request::builder();
        builder.method("GET");
        builder.uri(format!(
            "{}://{}/api/kv/{}",
            self.uri.scheme_part().unwrap(),
            self.uri.authority_part().unwrap(),
            key
        ));
        if let Some(secret) = &self.secret {
            builder.header(
                "Authorization",
                format!("Bearer {}", self.get_token(secret)),
            );
        }
        let req = builder.body(Body::empty()).unwrap();
        let res = self.client.request(req).await.context(RequestFailed {})?;
        let status = res.status();
        let mime_type = res
            .headers()
            .get(header::CONTENT_TYPE)
            .or(Some(&HeaderValue::from_static("application/octet-stream")))
            .unwrap()
            .to_str()
            .context(InvalidMimeType)?
            .to_owned();

        let mut body = res.into_body();
        let mut bytes = Vec::new();
        while let Some(next) = body.next().await {
            let chunk = next.context(RequestFailed {})?;
            bytes.extend(chunk);
        }

        match status.as_u16() {
            200 => Ok(Some(KvGetResponse {
                data: bytes,
                mime_type: mime_type,
            })),
            401 => Err(LucidClientError::Unauthorized),
            404 => Ok(None),
            code => Err(LucidClientError::InvalidStatus { code: code }),
        }
    }

    pub async fn put(&self, key: &str, value: Vec<u8>) -> Result<KvPutResponse, LucidClientError> {
        let mut builder = Request::builder();
        builder.method("PUT");
        builder.uri(format!(
            "{}://{}/api/kv/{}",
            self.uri.scheme_part().unwrap(),
            self.uri.authority_part().unwrap(),
            key
        ));
        if let Some(secret) = &self.secret {
            builder.header(
                "Authorization",
                format!("Bearer {}", self.get_token(secret)),
            );
        }
        let req = builder.body(Body::from(value)).unwrap();
        let res = self.client.request(req).await.context(RequestFailed {})?;
        let status = res.status();

        match status.as_u16() {
            200 => Ok(KvPutResponse::Updated),
            201 => Ok(KvPutResponse::Created),
            401 => Err(LucidClientError::Unauthorized),
            code => Err(LucidClientError::InvalidStatus { code: code }),
        }
    }
    pub async fn delete(&self, key: &str) -> Result<bool, LucidClientError> {
        let mut builder = Request::builder();
        builder.method("DELETE");
        builder.uri(format!(
            "{}://{}/api/kv/{}",
            self.uri.scheme_part().unwrap(),
            self.uri.authority_part().unwrap(),
            key
        ));
        if let Some(secret) = &self.secret {
            builder.header(
                "Authorization",
                format!("Bearer {}", self.get_token(secret)),
            );
        }
        let req = builder.body(Body::empty()).unwrap();
        let res = self.client.request(req).await.context(RequestFailed {})?;
        let status = res.status();

        match status.as_u16() {
            200 => Ok(true),
            401 => Err(LucidClientError::Unauthorized),
            404 => Ok(false),
            code => Err(LucidClientError::InvalidStatus { code: code }),
        }
    }

    fn get_token(&self, secret: &str) -> String {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                iss: String::default(),
                sub: String::default(),
                iat: Utc::now().timestamp(),
                exp: (Utc::now() + Duration::hours(1)).timestamp(),
            },
            secret.as_bytes(),
        )
        .unwrap()
    }
}

pub struct KvGetResponse {
    pub data: Vec<u8>,
    pub mime_type: String,
}

pub enum KvPutResponse {
    Created,
    Updated,
}

#[derive(Debug, Snafu)]
pub enum LucidClientError {
    #[snafu(display("Request failed: {}", source))]
    RequestFailed { source: hyper::Error },
    #[snafu(display("Invalid status code: {}", code))]
    InvalidStatus { code: u16 },
    #[snafu(display("Invalid mime type: {}", source))]
    InvalidMimeType { source: ToStrError },
    #[snafu(display("Request unauthorized"))]
    Unauthorized,
    #[snafu(display("Invalid URI"))]
    InvalidUri,
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}
