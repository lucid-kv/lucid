use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use bytes::Buf;
use jsonwebtoken::Validation;
use snafu::Snafu;
use warp::{
    self, filters, fs,
    http::{Response, StatusCode},
    path, reject, Filter, Rejection, Reply,
};

use crate::configuration::{Claims, Configuration};
use crate::kvstore::KvStore;

#[derive(Serialize, Deserialize)]
struct JsonMessage {
    message: String,
}

pub struct Server {
    configuration: Arc<RwLock<Configuration>>,
}

impl Server {
    pub fn new(configuration: Arc<RwLock<Configuration>>) -> Server {
        Server { configuration }
    }

    pub async fn run(&self) {
        let configuration = self.configuration.read().unwrap();

        let mut encryption_key = None;
        if configuration.encryption.enabled {
            if configuration.encryption.private_key.is_empty() {
                panic!("The private key must be filled.");
            }
            else if configuration.encryption.iv.is_empty() {
                panic!("The initialization vector must be filled.")
            }
            else {
                encryption_key = Some([
                    configuration.encryption.private_key.as_str(),
                    configuration.encryption.iv.as_str(),
                ]);
            }
        }
        let store = Arc::new(KvStore::new(encryption_key));
        let store = warp::any().map(move || store.clone());

        let config = self.configuration.clone();
        let config = warp::any().map(move || config.clone());

        let auth = warp::header::optional::<String>("authorization")
            .and(config.clone())
            .and_then(verify_auth)
            .untuple_one();

        let webui_enabled = config.clone().and_then(check_webui).untuple_one();

        let api_kv_key_path = path!("api" / "kv" / String).and(path::end());
        let api_kv_key = auth.and(
            warp::get()
                .and(store.clone())
                .and(api_kv_key_path)
                .and_then(get_key)
                .or(warp::put()
                    .and(store.clone())
                    .and(config.clone())
                    .and(api_kv_key_path)
                    .and(filters::body::content_length_limit(
                        configuration.http.request_size_limit,
                    ))
                    .and(filters::body::content_length_limit(
                        configuration.store.max_limit,
                    ))
                    .and(warp::body::aggregate())
                    .and_then(put_key))
                .or(warp::delete()
                    .and(store.clone())
                    .and(api_kv_key_path)
                    .and_then(delete_key))
                .or(warp::head()
                    .and(store.clone())
                    .and(api_kv_key_path)
                    .and_then(find_key))
                .or(warp::patch()
                    .and(store.clone())
                    .and(api_kv_key_path)
                    .and(filters::body::content_length_limit(
                        configuration.http.request_size_limit,
                    ))
                    .and(filters::body::json())
                    .and_then(patch_key)),
        );

        const WELCOME_PAGE: &'static str = r#"<!DOCTYPE html><html lang="en"> <head> <meta charset="utf-8" /> <meta name="viewport" content="width=device-width, initial-scale=1.0"> <link rel="dns-prefetch" href="//fonts.googleapis.com"> <link rel="preconnect" href="https://fonts.gstatic.com/" crossorigin></lien> <link href="https://fonts.googleapis.com/css?family=Noto+Sans:400,900&display=swap" rel="stylesheet"> <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.11.2/css/all.css" integrity="sha256-46qynGAkLSFpVbEBog43gvNhfrOj+BmwXdxFgVK/Kvc=" crossorigin="anonymous" /> <link rel="preload" href="https://avatars3.githubusercontent.com/u/56896360" as="image"> <meta name="title" content="Lucid KV"> <meta name="description" content="Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features. 🦀"> <meta name="keywords" content="lucid,kv,lucid kv,key-value,kv store,store,key-value store,lucid kv store,http,rest api,http kv store,key-value as a service"> <meta name="robots" content="index, follow"> <meta name="language" content="English"> <meta name="author" content="https://github.com/lucid-kv"> <meta name="twitter:site" content="lucid_kv" /> <meta name="twitter:title" content="Lucid KV" /> <meta name="twitter:description" content="Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features. 🦀" /> <meta name="twitter:image" content="https://lucid-kv.store/preview.jpg" /> <meta name="twitter:card" content="summary_large_image" /> <meta property="og:title" content="Lucid KV" /> <meta property="og:description" content="Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features. 🦀" /> <meta property="og:image" content="https://image" /> <meta property="og:url" content="https://lucid-kv.store" /> <meta property="og:site_name" content="Lucid KV" /> <meta property="og:type" content="article" /> <meta name="theme-color" content="\#1d1b26"> <link rel="icon" type="image/png" href="https://avatars3.githubusercontent.com/u/56896360" /> <title>Lucid KV | High performance and distributed KV store w/ REST API. 🦀</title> <style> body { background-color: #FFFFFF; font-family: "Inter UI", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"; margin: 0; font-size: 1rem; font-weight: 400; line-height: 1.5; color: #1d1b26; text-align: left; } .showcase { font-family: 'Noto Sans', sans-serif; display: flex; justify-content: center; align-items: center; flex-direction: column; height: 95vh; } .showcase img { height: 30vh; -webkit-filter: drop-shadow(5px 5px 5px rgba(17, 24, 20, .21)); filter: drop-shadow(5px 5px 5px rgba(17, 24, 20, .21)); animation: float 6s ease-in-out infinite; } .showcase h1 { font-weight: 900; font-size: 60pt; margin: 20px 20px 0; } .showcase h2 { font-size: 30pt; margin: 0; font-weight: 400; } .showcase p { font-size: 17pt; text-align: center; } .showcase div { margin-top: 30px; } .btn { border: 1px solid #1d1b26; padding: 12px 15px; color: #1d1b26; text-decoration: none; border-radius: 3px; margin-top: 15px; text-align: center; } .btn.fill { background-color: #1d1b26; color: #FFFFFF; } .showcase .btn:not(.fill) { margin-left: 10px; } @keyframes float { 0% { -webkit-transform: translatey(0px); transform: translatey(0px); } 50% { -webkit-transform: translatey(-20px); transform: translatey(-20px); } 100% { -webkit-transform: translatey(0px); transform: translatey(0px); } } @media (max-width: 991px) { .showcase { height: 100vh; } .showcase img { height: initial; width: 60vw; } .showcase h1 { font-size: 40pt; } .showcase p { font-size: 12pt; padding: 0 20px; } .showcase div { display: flex; margin-top: initial; flex-direction: column; } .showcase .btn:not(.fill) { margin-left: initial; } } </style> </head> <body> <div class="showcase"> <a href="https://github.com/lucid-kv/lucid" target="_blank"> <img src="https://avatars3.githubusercontent.com/u/56896360" alt="lucid-kv logo" /> </a> <h1>Lucid KV</h1> <p>Lucid is an high performance and distributed KV store<br />accessible through an HTTP API. 🦀</p> <div> <a class="btn fill" href="https://medium.com/@clintnetwork/lucid-an-http-key-value-store-c0e734586e26"><i class="fas fa-info-circle"></i>&nbsp; Read About Lucid KV</a> <a class="btn " href="https://docs.lucid-kv.store/"><i class="fas fa-book"></i>&nbsp; Official Documentation</a> </div> </div> </body></html>"#;

        let webui = warp::path::end()
            .and(fs::file("assets/webui/dist/index.html"))
            .or(fs::dir("assets/webui/dist"))
            .and(webui_enabled)
            .or(warp::get().map(|| warp::reply::html(WELCOME_PAGE)))
            .and(warp::path::end());

        let robots = warp::path("robots.txt")
            .and(path::end())
            .and(warp::get().map(|| "User-agent: *\nDisallow: /"));

        let log = warp::log("lucid::Server");

        let cors = warp::cors()
            .allow_methods(vec!["HEAD", "GET", "PUT", "POST", "PATCH", "DELETE"])
            .allow_any_origin();

        let routes = api_kv_key
            .or(webui)
            .or(robots)
            .recover(process_error)
            .with(warp::reply::with::header(
                "Server",
                format!("Lucid v{}", crate_version!()),
            ))
            .with(cors)
            .with(log);

        let instance = warp::serve(routes);
        if configuration.general.use_ssl {
            let bind_endpoint = SocketAddr::from((
                configuration.general.bind_address,
                configuration.general.port_ssl,
            ));
            info!(
                "Running Lucid server on {} | PID: {}",
                bind_endpoint,
                std::process::id()
            );
            info!("Lucid API Endpoint: https://{}/api/", bind_endpoint);
            info!(
                "SSL Certificate: {}",
                &configuration.general.ssl_certificate
            );
            info!(
                "SSL Private Key: {}",
                &configuration.general.ssl_certificate_key
            );
            info!("Use Ctrl+C to stop the server.");
            instance
                .tls()
                .cert_path(&configuration.general.ssl_certificate)
                .key_path(&configuration.general.ssl_certificate_key)
                .bind((
                    configuration.general.bind_address,
                    configuration.general.port_ssl,
                ))
                .await;
        } else {
            let bind_endpoint = SocketAddr::from((
                configuration.general.bind_address,
                configuration.general.port,
            ));
            info!(
                "Running Lucid server on {} | PID: {}",
                bind_endpoint,
                std::process::id()
            );
            info!("Lucid API Endpoint: http://{}/api/", bind_endpoint);
            info!("Use Ctrl+C to stop the server.");
            instance
                .bind((
                    configuration.general.bind_address,
                    configuration.general.port,
                ))
                .await;
        }
    }
}

async fn get_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(value) = store.get(key) {
        Ok(Response::builder()
            .header("Content-Type", value.mime_type)
            .body(value.data))
    } else {
        Err(reject::custom(Error::KeyNotFound))
    }
}

async fn put_key(
    store: Arc<KvStore>,
    config: Arc<RwLock<Configuration>>,
    key: String,
    body: impl Buf,
) -> Result<impl Reply, Rejection> {
    if body.remaining() == 0 {
        Err(reject::custom(Error::MissingBody))
    } else if body.bytes().len() as u64 > config.read().unwrap().store.max_limit {
        Err(reject::custom(Error::ValueSizeLimit {
            max_limit: config.read().unwrap().store.max_limit,
        }))
    } else {
        if let Some(_) = store.set(key, body.bytes().to_vec()) {
            Ok(warp::reply::json(&JsonMessage {
                message: "The specified key was successfully updated.".to_string(),
            }))
        } else {
            Ok(warp::reply::json(&JsonMessage {
                message: "The specified key was successfully created.".to_string(),
            }))
        }
    }
}

async fn delete_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key.clone()) {
        (*store).drop(key);
        Ok(warp::reply::json(&JsonMessage {
            message: "The specified key and it's data was successfully deleted".to_string(),
        }))
    } else {
        Err(reject::custom(Error::KeyNotFound))
    }
}

async fn find_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(value) = store.get(key) {
        Ok(Response::builder()
            .header("Content-Type", value.mime_type)
            .body(value.data))
    } else {
        Err(reject::custom(Error::KeyNotFound))
    }
}

#[derive(Debug, Deserialize)]
struct PatchValue {
    operation: String,
}
async fn patch_key(
    store: Arc<KvStore>,
    key: String,
    patch_value: PatchValue,
) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key.clone()) {
        match patch_value.operation.to_lowercase().as_str() {
            "lock" => {
                store.switch_lock(key.to_string(), true);
                Ok(warp::reply::json(&JsonMessage {
                    message: "The specified key was successfully locked.".to_string(),
                }))
            }
            "unlock" => {
                store.switch_lock(key.to_string(), false);
                Ok(warp::reply::json(&JsonMessage {
                    message: "The specified key was successfully unlocked.".to_string(),
                }))
            }
            "increment" => {
                store.increment_or_decrement(key.to_string(), 1.0);
                Ok(warp::reply::json(&JsonMessage {
                    message: "The specified key was successfully incremented.".to_string(),
                }))
            }
            "decrement" => {
                store.increment_or_decrement(key.to_string(), -1.0);
                Ok(warp::reply::json(&JsonMessage {
                    message: "The specified key was successfully decremented.".to_string(),
                }))
            }
            _ => Err(reject::custom(Error::InvalidOperation {
                operation: patch_value.operation,
            })),
        }
    } else {
        Err(reject::custom(Error::KeyNotFound))
    }
}

async fn verify_auth(
    auth_header: Option<String>,
    config: Arc<RwLock<Configuration>>,
) -> Result<(), Rejection> {
    let config = config.read().unwrap();
    if config.authentication.enabled {
        if let Some(auth_header) = auth_header {
            if let Ok(_bearer) = jsonwebtoken::decode::<Claims>(
                auth_header.trim_start_matches("Bearer "),
                config.authentication.secret_key.as_ref(),
                &Validation::default(),
            ) {
                Ok(())
            } else {
                Err(reject::custom(Error::InvalidJwtToken))
            }
        } else {
            Err(reject::custom(Error::MissingAuthHeader))
        }
    } else {
        Ok(())
    }
}

async fn check_webui(config: Arc<RwLock<Configuration>>) -> Result<(), Rejection> {
    let config = config.read().unwrap();
    if config.webui.enabled {
        Ok(())
    } else {
        Err(reject::not_found())
    }
}

async fn process_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = err.find::<Error>() {
        let code = match err {
            Error::MissingBody => StatusCode::BAD_REQUEST,
            Error::MissingParameter { .. } => StatusCode::BAD_REQUEST,
            Error::MissingAuthHeader => StatusCode::UNAUTHORIZED,
            Error::KeyNotFound => StatusCode::NOT_FOUND,
            Error::InvalidOperation { .. } => StatusCode::BAD_REQUEST,
            Error::InvalidJwtToken => StatusCode::UNAUTHORIZED,
            Error::ValueSizeLimit { .. } => StatusCode::BAD_REQUEST,
        };
        let json = warp::reply::json(&JsonMessage {
            message: err.to_string(),
        });
        Ok(warp::reply::with_status(json, code))
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        let code = StatusCode::METHOD_NOT_ALLOWED;
        let json = warp::reply::json(&JsonMessage {
            message: "Method not allowed.".to_string(),
        });
        Ok(warp::reply::with_status(json, code))
    } else if let Some(_) = err.find::<reject::PayloadTooLarge>() {
        let code = StatusCode::METHOD_NOT_ALLOWED;
        let json = warp::reply::json(&JsonMessage {
            message: "Request payload is too long.".to_string(), // TODO: find a way to format the limit into this string
        });
        Ok(warp::reply::with_status(json, code))
    } else {
        Err(err)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Missing request body."))]
    MissingBody,
    #[snafu(display("Missing \"{}\" parameter.", parameter))]
    MissingParameter { parameter: String },
    #[snafu(display("Missing Authorization header."))]
    MissingAuthHeader,
    #[snafu(display("The specified key does not exist."))]
    KeyNotFound,
    #[snafu(display("Invalid Operation \"{}\".", operation))]
    InvalidOperation { operation: String },
    #[snafu(display("Invalid JWT token in Authorization header."))]
    InvalidJwtToken,
    #[snafu(display("The maximum allowed value size is {} bytes.", max_limit))]
    ValueSizeLimit { max_limit: u64 },
}

impl reject::Reject for Error {}
