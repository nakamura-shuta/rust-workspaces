//! Project-specific model definitions
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Foo {
    pub msg: String,
}
