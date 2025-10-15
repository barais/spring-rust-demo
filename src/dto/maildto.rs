use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Local struct for OpenAPI compatibility
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MailResponse {
    pub success: bool,
    pub message: String,
}
