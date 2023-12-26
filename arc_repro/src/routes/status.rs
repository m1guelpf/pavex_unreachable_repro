use pavex::http::StatusCode;
use std::sync::Arc;

use crate::SharedDep;

pub fn ping(dep: Arc<SharedDep>) -> StatusCode {
    dbg!(dep);
    StatusCode::OK
}
