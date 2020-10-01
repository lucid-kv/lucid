use std::{net::SocketAddr, sync::RwLock};

use bytes::{Buf, Bytes};
use jsonwebtoken::Validation;
use snafu::Snafu;
use std::sync::Arc;
use tokio::{
    stream::{Stream, StreamExt},
    sync::broadcast,
};
use warp::{
    self, filters, fs,
    http::{Response, StatusCode},
    path, reject, Rejection, Reply,
};
use warp::{sse::ServerSentEvent, Filter};

use crate::configuration::{Claims, Configuration};
use crate::kvstore::KvStore;

#[derive(Debug, Clone)]
pub struct SseMessage {
    key: String,
    value: String,
}

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
            } else if configuration.encryption.iv.is_empty() {
                panic!("The initialization vector must be filled.")
            } else {
                encryption_key = Some([
                    configuration.encryption.private_key.as_str(),
                    configuration.encryption.iv.as_str(),
                ]);
            }
        }
        let store = Arc::new(KvStore::new(encryption_key));
        let event_tx = Arc::new(broadcast::channel(512).0); // TODO: Specify in configuration (maybe?)

        let instance = warp::serve(routes_filter(store, event_tx, self.configuration.clone()));
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

pub fn routes_filter(
    store: Arc<KvStore>,
    event_tx: Arc<broadcast::Sender<SseMessage>>,
    config: Arc<RwLock<Configuration>>,
) -> impl Filter<Extract = (impl Reply,)> + Clone + Send + Sync + 'static {
    let configuration = config.read().unwrap();

    let store = warp::any().map(move || store.clone());
    let event_tx = warp::any().map(move || event_tx.clone());

    let config = config.clone();
    let config = warp::any().map(move || config.clone());

    let auth = warp::header::optional::<String>("authorization")
        .and(config.clone())
        .and_then(verify_auth)
        .untuple_one();

    let webui_enabled = config.clone().and_then(check_webui).untuple_one();

    let sse_enabled = config.clone().and_then(check_sse).untuple_one();

    let mime = warp::header::optional::<String>("content-type");

    let api_kv_key_path = path!("api" / "kv" / String)
        .and(path::end());

    let api_kv_key = auth.clone().and(
        warp::get()
            .and(store.clone())
            .and(api_kv_key_path)
            .and_then(get_key)
            .or(warp::put()
                .and(store.clone())
                .and(event_tx.clone())
                .and(config.clone())
                .and(api_kv_key_path)
                .and(filters::body::content_length_limit(
                    configuration.http.request_size_limit,
                ))
                .and(filters::body::content_length_limit(
                    configuration.store.max_limit,
                ))
                .and(warp::body::bytes())
                .and(mime.clone())
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

    const WELCOME_PAGE: &'static str = include_str!("../assets/welcome.html");

    let webui = fs::file("assets/webui/dist/index.html")
        .or(fs::dir("assets/webui/dist"))
        .and(webui_enabled)
        .or(warp::get().map(move || warp::reply::html(WELCOME_PAGE)))
        .and(warp::path::end());

    let robots = warp::path("robots.txt")
        .and(path::end())
        .and(warp::get().map(|| "User-agent: *\nDisallow: /api"));

    let cors = warp::cors()
        .allow_methods(vec!["HEAD", "GET", "PUT", "POST", "PATCH", "DELETE"])
        .allow_any_origin();

    let health = path!("health")
        .map(|| StatusCode::OK);

    let sse = warp::path("notifications")
        .and(warp::get())
        .and(event_tx)
        .and(auth)
        .and(sse_enabled)
        .map(|event_tx: Arc<broadcast::Sender<SseMessage>>| {
            let stream = sse_event_stream(event_tx.subscribe());
            warp::sse::reply(warp::sse::keep_alive().stream(stream))
        });

    api_kv_key
        .or(webui)
        .or(sse)
        .or(robots)
        .or(health)
        .recover(process_error)
        .with(warp::reply::with::header(
            "Server",
            format!("Lucid v{}", crate_version!()),
        ))
        .with(cors)
        .with(warp::log("lucid::server"))
}

async fn put_key(
    store: Arc<KvStore>,
    event_tx: Arc<broadcast::Sender<SseMessage>>,
    config: Arc<RwLock<Configuration>>,
    key: String,
    body: Bytes,
    mime: Option<String>,
) -> Result<impl Reply, Rejection> {
    if body.remaining() == 0 {
        Err(reject::custom(Error::MissingBody))
    } else if body.bytes().len() as u64 > config.read().unwrap().store.max_limit {
        Err(reject::custom(Error::ValueSizeLimit {
            max_limit: config.read().unwrap().store.max_limit,
        }))
    } else {
        match store.set(key.clone(), body.to_vec(), mime) {
            Some(kv_element) => {
                if kv_element.locked {
                    Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key cannot be updated, it is currently locked.".to_string(),
                    }), StatusCode::FORBIDDEN))
                } else {
                    if config.read().unwrap().sse.enabled {
                        // TODO: Send an SSE fallaback message for binary data
                        match String::from_utf8((&body).bytes().to_vec()) {
                            Ok(byte_to_string) => {
                                event_tx
                                    .send(SseMessage {
                                        key: key.clone(),
                                        value: byte_to_string,
                                    })
                                    .map_err(|error| {
                                        error!("Unable to broadcast this key: {:?}", &error.0.key)
                                    })
                                    .ok();
                            }
                            Err(error) => warn!("Unable to broadcast binary data, {}", error),
                        }
                    }
                    Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key was successfully updated.".to_string(),
                    }), StatusCode::OK))
                }
            }
            None => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                message: "The specified key was successfully created.".to_string(),
            }), StatusCode::CREATED))
        }
    }
}

async fn get_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    match store.get(key) {
        Some(value) => Ok(Response::builder()
            .header("Content-Type", value.mime_type)
            .body(value.data)),
        None => Err(reject::custom(Error::KeyNotFound))
    }
}

async fn find_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    match store.get(key) {
        Some(value) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", value.mime_type)
            .header("Content-Length", value.data.len())
            .header("Last-Modified", value.updated_at.format("%a, %d %b %Y %T GMT").to_string())
            .body("")
            .unwrap()),
        None => Err(reject::custom(Error::KeyNotFound))
    }
}

async fn delete_key(store: Arc<KvStore>, key: String) -> Result<impl Reply, Rejection> {
    match store.get(key.clone()) {
        Some(_) => {
            (*store).drop(key);
            Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                message: "The specified key and it's data was successfully deleted.".to_string(),
            }), StatusCode::NO_CONTENT))
        }
        None => Err(reject::custom(Error::KeyNotFound))
    }
}

#[derive(Debug, Deserialize)]
struct PatchValue {
    operation: String,
    value: Option<String>,
}
async fn patch_key(
    store: Arc<KvStore>,
    key: String,
    patch_value: PatchValue,
) -> Result<impl Reply, Rejection> {
    if let Some(_) = store.get(key.clone()) {
        match patch_value.operation.to_lowercase().as_str() {
            "lock" => {
                match store.switch_lock(key.to_string(), true) {
                    true => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key was successfully locked.".to_string(),
                    }), StatusCode::OK)),
                    false => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key is already locked.".to_string(),
                    }), StatusCode::CONFLICT))
                }
            }
            "unlock" => {
                match store.switch_lock(key.to_string(), false) {
                    true => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key was successfully unlocked.".to_string(),
                    }), StatusCode::OK)),
                    false => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key is not currently locked.".to_string(),
                    }), StatusCode::CONFLICT))
                }
            }
            "increment" => {
                match store.increment_or_decrement(key.to_string(), 1.0) {
                    true => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key was successfully incremented.".to_string(),
                    }), StatusCode::OK)),
                    false => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key is not a valid numeric value.".to_string(),
                    }), StatusCode::BAD_REQUEST))
                }
            }
            "decrement" => {
                match store.increment_or_decrement(key.to_string(), -1.0) {
                    true => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key was successfully decremented.".to_string(),
                    }), StatusCode::OK)),
                    false => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "The specified key is not a valid numeric value.".to_string(),
                    }), StatusCode::BAD_REQUEST))
                }
            }
            "ttl" => {
                match patch_value.value {
                    Some(value) => {
                        if let Ok(ttl) = value.parse::<i64>() {
                            match store.set_expiration(key.clone(), ttl) {
                                Some(expiration_date) => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                                    message: format!("The expiration is successsfully setup, the key will expire at {}.", expiration_date).to_string(),
                                }), StatusCode::OK)),
                                None => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                                    message: "Unable to set the expiration for the specified key.".to_string(),
                                }), StatusCode::BAD_REQUEST))
                            }
                        }
                        else {
                            Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                                message: "Unrecognized value for expiration, you need to use a numeric value without decimal.".to_string(),
                            }), StatusCode::BAD_REQUEST))
                        }
                    }
                    None => Ok(warp::reply::with_status(warp::reply::json(&JsonMessage {
                        message: "Missing value for expiration.".to_string(),
                    }), StatusCode::BAD_REQUEST))
                }
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

async fn check_sse(config: Arc<RwLock<Configuration>>) -> Result<(), Rejection> {
    let config = config.read().unwrap();
    if config.sse.enabled {
        Ok(())
    } else {
        Err(reject::not_found())
    }
}

fn sse_event_stream(
    event_rx: broadcast::Receiver<SseMessage>,
) -> impl Stream<Item = Result<impl ServerSentEvent + Send + 'static, warp::Error>> + Send + 'static
{
    event_rx.filter_map(move |msg| match msg {
        Ok(msg) => Some(Ok((warp::sse::event(msg.key), warp::sse::data(msg.value)))),
        Err(broadcast::RecvError::Lagged(lag)) => {
            warn!("SSE stream lagged, {} events lost", lag);
            None
        }
        Err(broadcast::RecvError::Closed) => unreachable!(),
    })
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
