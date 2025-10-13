use spring::tracing;
use spring_mail::Response;
use spring_web::{
    axum::Json, axum::http::StatusCode, axum::response::IntoResponse, extractor::Component,
    extractor::Path,
};

use crate::dto::userdto::{UserDto, UserInput, UserResponse};
use crate::service::mailservice::MailService;
use crate::service::userservice::UserService;
use spring_utoipa::utoipa;
use spring_utoipa::utoipa::OpenApi;
use spring_web::{get, route};

#[derive(OpenApi)]
#[openapi(paths(hello_world,hello, get_all_user, create_user,get_user_by_id), components(schemas(UserResponse, UserInput,UserDto)), tags(
    (name = "example", description = "Example APIs")
))]
pub struct ApiDoc;

// The get macro specifies the Http Method and request path.
// spring-rs also provides other standard http method macros such as post, delete, patch, etc.
#[utoipa::path(get, path = "/")]
#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

// You can also use the route macro to specify the Http Method and request path.
// Path extracts parameters from the HTTP request path
#[utoipa::path(get, path = "/hello/{name}")]
#[route("/hello/{name}", method = "GET", method = "POST")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}

#[utoipa::path(get, path = "/user/all",description =  "Get all user information" , responses(
    (status = 200, description = "User found", body = UserDto),
    (status = 404, description = "User not found")))]
#[route("/user/all", method = "GET")]
async fn get_all_user() -> Result<Json<Vec<UserDto>>, StatusCode> {
    let mut users: Vec<UserDto> = Vec::new();
    let userdto = UserDto {
        id: 123,
        name: "John".to_string(),
        firstname: "Doe".to_string(),
        age: Some(30),
    };
    users.push(userdto);

    Ok(Json(users))
}

#[utoipa::path(get, path = "/user/{id}",description =  "Get user information" , responses(
    (status = 200, description = "User found", body = UserDto),
    (status = 404, description = "User not found")), params(
        ("id" = u64, Path, description = "user database id to get User for"),
    )

)]
#[route("/user/{id}", method = "GET")]
async fn get_user_by_id(
    Component(userservice): Component<UserService>,
    Path(id): Path<i64>,
) -> Result<Json<UserDto>, StatusCode> {
    Ok(Json(userservice.get_user(id).await))
}

#[route("/sendemail", method = "GET")]
async fn sendemail(
    Component(mail_service): Component<MailService>,
) -> Result<Json<Response>, StatusCode> {
    Ok(Json(
        mail_service
            .send_mail("demo1@demo.com".to_string())
            .await
            .unwrap(),
    ))
}

#[utoipa::path(post, path = "/user",description =  "post user information" , 
    responses(
    (status = 200, description = "User created", body = UserResponse),
    (status = 404, description = "User not found")),
    request_body = UserInput
    )]
#[route("/user", method = "POST")]
async fn create_user(Json(payload): Json<UserInput>) -> Result<Json<UserResponse>, StatusCode> {
    let s = format!(
        "Received user: {} ({} years old)",
        payload.name, payload.age
    );
    tracing::info!(s);

    Ok(Json(UserResponse {
        id: 123,
        message: "User data retrieved successfully.".to_string(),
    }))
}
