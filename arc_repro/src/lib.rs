mod blueprint;
pub mod configuration;
pub mod routes;
pub mod telemetry;

use std::sync::Arc;

pub use blueprint::blueprint;

#[derive(Debug)]
pub struct ExampleDependency;

pub fn dependency_with_arc() -> std::sync::Arc<ExampleDependency> {
    Arc::new(ExampleDependency {})
}
