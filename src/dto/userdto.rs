use schemars::JsonSchema;
use utoipa::ToSchema;
use serde::Serialize;
use serde::Deserialize;


#[derive(Debug,Serialize,JsonSchema,ToSchema)]
pub struct UserResponse {
    pub id: u32,
    pub message: String,
}
 

#[derive(Deserialize,Debug,JsonSchema,ToSchema)]
pub struct UserInput {
    pub name: String,
    pub age: u8,
}


#[derive(Deserialize,Serialize,Debug,JsonSchema,ToSchema)]
pub struct UserDto {
   pub id: i64,
   pub name: String,
   pub firstname: String,
   pub age: Option<i32>,
}