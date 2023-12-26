mod blueprint;
pub mod configuration;
pub mod routes;
pub mod telemetry;
use std::{future::IntoFuture, sync::Arc};

pub use blueprint::blueprint;
use pavex::{middleware::Next, response::Response};

#[derive(Debug)]
pub struct SharedDep {}

pub fn build_shared_dep() -> Arc<SharedDep> {
    Arc::new(SharedDep {})
}

#[derive(Debug)]
pub struct MiddlewareDep {
    shared_dep: Arc<SharedDep>,
}

impl MiddlewareDep {
    pub fn new(shared_dep: Arc<SharedDep>) -> Self {
        Self { shared_dep }
    }
}

pub async fn middleware<C>(middleware_dep: &MiddlewareDep, next: Next<C>) -> Response
where
    C: IntoFuture<Output = Response> + Send,
    C::IntoFuture: Send,
{
    next.into_future().await
}
