use crate::{Context, Response, handler::{not_found_handler}, routes::response::IntoResponse};
use async_trait::async_trait;
use futures::future::Future;
use hyper::{Method};
use route_recognizer::{Match, Params, Router as InternalRouter};
use std::collections::HashMap;


pub struct Router {
    method_map: HashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            method_map: HashMap::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::GET)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn post(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::POST)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn put(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::PUT)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn delete(&mut self, path: &str, handler: Box<dyn Handler>) {
        self.method_map
            .entry(Method::DELETE)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    // This is where we actually match HTTP requests and routes to handlers
    pub fn route(&self, path: &str, method: &Method) -> RouterMatch<'_> {
        if let Some(Match { handler, params }) = self
            .method_map
            .get(method)
            .and_then(|r| r.recognize(path).ok())
        {
            RouterMatch {
                // handler is a &Box<dyn Handler>, and need to transform into Box<dyn Handler>
                // so we essentially
                handler: &**handler,
                params,
            }
        } else {
            RouterMatch {
                handler: &not_found_handler,
                params: Params::new(),
            }
        }
    }
}

// 'a to only have handler alive in memory while request is active
pub struct RouterMatch<'a> {
    pub handler: &'a dyn Handler,
    pub params: Params,
}

// In order to put references to handler functions, in order to pass on Context
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, context: Context) -> Response;
}

// Implementing the trait for functions F, that return Futures
#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn(Context) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: IntoResponse,
{
    async fn invoke(&self, context: Context) -> Response {
        (self)(context).await.into_response()
    }
}