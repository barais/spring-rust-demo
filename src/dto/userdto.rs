use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use spring::config::Configurable;
// use utoipa::ToSchema;
use validator::Validate;


#[derive(Debug, Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: i64,
    pub message: String,
}

#[derive(Deserialize, Validate,Debug, JsonSchema)]
pub struct UserInput {
    #[validate(length(min = 3,max = 30, message = "Name is too long or too short"))]
    pub name: String,
    pub firstname: String,
    pub age: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema,Clone)]
pub struct UserDto {
    pub id: Option<i64>,
    pub name: String,
    pub firstname: String,
    pub age: Option<i32>,
}

#[derive(Configurable, JsonSchema, Deserialize)]
#[config_prefix = "custom"]
pub struct CustomConfig {
    pub user_info_detail: String,
}
