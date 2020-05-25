use std::sync::{Arc, RwLock};

use hyper::StatusCode;
use serde_json::Value;
use tokio::sync::broadcast;
use warp::{Filter, Reply};

use lucid::{
    configuration::{Configuration, ServerSentEvent},
    kvstore::KvStore,
    server::routes_filter,
};

fn create_routes_filter() -> impl Filter<Extract = (impl Reply,)> + Clone + Send + Sync + 'static {
    let store = Arc::new(KvStore::new(None));
    let event_tx = Arc::new(broadcast::channel(512).0);
    let config = Arc::new(RwLock::new(Configuration {
        sse: ServerSentEvent { enabled: true },
        ..Default::default()
    }));
    routes_filter(store, event_tx, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set() {
        let routes = create_routes_filter();
        let reply = warp::test::request()
            .method("PUT")
            .path("/api/kv/foo")
            .body(b"bar")
            .filter(&routes)
            .await
            .unwrap();

        let response = reply.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(serde_json::from_slice::<Value>(&body).is_ok());
    }

    #[tokio::test]
    async fn empty_get() {
        let routes = create_routes_filter();
        let reply = warp::test::request()
            .path("/api/kv/foo")
            .filter(&routes)
            .await
            .unwrap();

        let response = reply.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn set_get() {
        let routes = create_routes_filter();
        let set_reply = warp::test::request()
            .method("PUT")
            .path("/api/kv/foo")
            .body(b"bar")
            .filter(&routes)
            .await
            .unwrap();

        let set_response = set_reply.into_response();
        assert_eq!(set_response.status(), StatusCode::CREATED);

        let get_reply = warp::test::request()
            .path("/api/kv/foo")
            .filter(&routes)
            .await
            .unwrap();

        let get_response = get_reply.into_response();
        assert_eq!(get_response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(get_response.into_body())
            .await
            .unwrap();
        assert_eq!(&*body, b"bar");
    }

    #[tokio::test]
    async fn empty_delete() {
        let routes = create_routes_filter();
        let reply = warp::test::request()
            .method("DELETE")
            .path("/api/kv/foo")
            .filter(&routes)
            .await
            .unwrap();

        let response = reply.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn set_delete() {
        let routes = create_routes_filter();
        let set_reply = warp::test::request()
            .method("PUT")
            .path("/api/kv/foo")
            .body(b"bar")
            .filter(&routes)
            .await
            .unwrap();

        let set_response = set_reply.into_response();
        assert_eq!(set_response.status(), StatusCode::CREATED);

        let delete_reply = warp::test::request()
            .method("DELETE")
            .path("/api/kv/foo")
            .filter(&routes)
            .await
            .unwrap();

        let delete_response = delete_reply.into_response();
        assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn sse_notifications() {
        let routes = create_routes_filter();
        let sse_reply = warp::test::request()
            .method("GET")
            .path("/notifications")
            .filter(&routes)
            .await
            .unwrap();

        let sse_response = sse_reply.into_response();
        assert_eq!(sse_response.status(), StatusCode::OK);

        // TODO: parse body and check if the events are correct
    }
}
