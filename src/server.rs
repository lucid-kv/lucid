use std::sync::Arc;
use std::sync::RwLock;

use bytes::Buf;
use jsonwebtoken::Validation;
use snafu::Snafu;
use warp::{self, filters, fs, http::StatusCode, path, Filter, Rejection, Reply};

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
    pub fn new() -> Server {
        Server {
            configuration: Arc::new(RwLock::new(Configuration::default())),
        }
    }

    pub fn configure(&mut self, configuration: Configuration) {
        *self.configuration.write().unwrap() = configuration;
    }

    pub fn run(&self) {
        let store = Arc::new(KvStore::new());
        let store = warp::any().map(move || store.clone());

        let config = self.configuration.clone();
        let config = warp::any().map(move || config.clone());

        let auth = warp::header::optional::<String>("authorization")
            .and(config.clone())
            .and_then(verify_auth)
            .untuple_one();

        let webui_enabled = config
            .clone()
            .and_then(|config: Arc<RwLock<Configuration>>| {
                if config.read().unwrap().webui.enabled {
                    Ok(())
                } else {
                    Err(warp::reject::not_found())
                }
            })
            .untuple_one();

        let configuration = self.configuration.read().unwrap();

        let api_kv_key_path = path!("api" / "kv" / String).and(path::end());
        let api_kv_key = auth
            .and(
                warp::get2()
                    .and(store.clone())
                    .and(api_kv_key_path)
                    .and_then(get_key)
                    .or(warp::put2()
                        .and(store.clone())
                        .and(config.clone())
                        .and(api_kv_key_path)
                        .and(filters::body::content_length_limit(
                            configuration.http.request_size_limit,
                        ))
                        .and(filters::body::content_length_limit(
                            configuration.store.max_limit,
                        ))
                        .and(warp::body::concat())
                        .and_then(put_key))
                    .or(warp::delete2()
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

        let webui = warp::path::end()
            .and(fs::file("webui/dist/index.html"))
            .or(fs::dir("webui/dist"))
            .and(webui_enabled);

        let robots = warp::path("robots.txt")
            .and(path::end())
            .and(warp::get2().map(|| "User-agent: *\nDisallow: /"));

        let log = warp::log("lucid::Server");

        let routes = api_kv_key
            .or(webui)
            .or(robots)
            .recover(process_error)
            .with(log);

        warp::serve(routes).run((
            configuration.general.bind_address,
            configuration.general.port,
        ), );
    }
}

fn get_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(value) = store.get(key) {
        Ok(value)
    } else {
        Err(warp::reject::custom(Error::KeyNotFound))
    }
}

fn put_key(
    store: Arc<KvStore>,
    config: Arc<RwLock<Configuration>>,
    key: String,
    body: filters::body::FullBody,
) -> Result<impl Reply, Rejection> {
    if body.remaining() == 0 {
        Err(warp::reject::custom(Error::MissingBody))
    } else if body.bytes().len() as u64 > config.read().unwrap().store.max_limit {
        Err(warp::reject::custom(Error::ValueSizeLimit {
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

fn delete_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key.clone()) {
        (*store).drop(key);
        Ok(warp::reply::json(&JsonMessage {
            message: "The specified key and it's data was successfully deleted".to_string(),
        }))
    } else {
        Err(warp::reject::custom(Error::KeyNotFound))
    }
}
fn find_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key) {
        Ok("")
    } else {
        Err(warp::reject::custom(Error::KeyNotFound))
    }
}

#[derive(Debug, Deserialize)]
struct PatchValue {
    operation: String,
}
fn patch_key(
    store: Arc<KvStore>,
    key: String,
    patch_value: PatchValue,
) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key.clone()) {
        match patch_value.operation.as_str() {
            "lock" | "unlock" => {
                let r = store.switch_lock(key.to_string(), true);
                println!("{}", r);
                Ok("")
            }
            _ => Err(warp::reject::custom(Error::InvalidOperation {
                operation: patch_value.operation,
            })),
        }
    } else {
        Err(warp::reject::custom(Error::KeyNotFound))
    }
}

fn verify_auth(
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
                Err(warp::reject::custom(Error::InvalidJwtToken))
            }
        } else {
            Err(warp::reject::custom(Error::MissingAuthHeader))
        }
    } else {
        Ok(())
    }
}

fn process_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = err.find_cause::<Error>() {
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
    } else if let Some(_) = err.find_cause::<warp::reject::MethodNotAllowed>() {
        let code = StatusCode::METHOD_NOT_ALLOWED;
        let json = warp::reply::json(&JsonMessage {
            message: "Method not allowed.".to_string(),
        });
        Ok(warp::reply::with_status(json, code))
    } else if let Some(_) = err.find_cause::<warp::reject::PayloadTooLarge>() {
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
