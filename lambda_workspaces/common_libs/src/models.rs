use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub command: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub req_id: String,
    pub msg: String,
}
