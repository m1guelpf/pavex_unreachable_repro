use std::sync::Arc;

use pavex::http::StatusCode;

use crate::ExampleDependency;

/// Respond with a `200 OK` status code to indicate that the server is alive
/// and ready to accept new requests.
pub fn ping(dep: Arc<ExampleDependency>) -> StatusCode {
    StatusCode::OK
}
