use std::{env, sync::Arc};
use redis::Commands;
use bytes::Bytes;
use dotenv::dotenv;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use route_recognizer::Params;

use crate::{controllers::lobby_controller::init_lobby_routes, router::Router};

pub mod controllers;
pub mod handler;
pub mod router;
pub mod routes;


#[tokio::main]
async fn main() {
    //Environment variables from .env
    dotenv().ok();

    // Redis connection
    //format - host:port
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    let mut redis_client = redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL");


    let mut router: Router = Router::new();

    // Init routes --------
    init_lobby_routes(&mut router);

    // -------------------

    let shared_router = Arc::new(router);

    // App state and making route service function
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            redis_client: redis_client.clone(),
            test_string: String::from("THis is a test"),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });


    let addr = "0.0.0.0:8080".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

// Actual route function
async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

// Help types

pub type Response = hyper::Response<hyper::Body>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

// Here we can hold database references and other stuff for handlers
// that can be initted at boot time
#[derive(Clone, Debug)]
pub struct AppState {
    redis_client: redis::Client,
    test_string: String,
}


#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    // To take the body of a request and give it in json format
    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}
