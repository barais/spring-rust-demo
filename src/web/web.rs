use spring::tracing;
use spring_web::extractor::Query;
use spring_web::{
    axum::Json, axum::http::StatusCode, axum::response::IntoResponse, extractor::Component,
    extractor::Path,extractor::Config,
};
use validator::Validate;

use crate::dto::maildto::MailResponse;
use crate::dto::userdto::{CustomConfig, UserDto, UserInput, UserResponse};
use crate::service::mailservice::MailService;
use crate::service::userservice::UserService;
use crate::web::jwt::Claims;
use crate::web::pagination::Pagination;
// use spring_utoipa::utoipa;
// use spring_utoipa::utoipa::OpenApi;
use spring_web::{ get, get_api, post_api};


#[get_api("/")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

#[get_api("/hello/{name}")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}

#[get_api("/user/all")]
async fn get_all_user(pagination: Query<Pagination>,
Component(userservice): Component<UserService>,
) -> Result<Json<Vec<UserDto>>, StatusCode> {
    let users = userservice.get_alluser(pagination.0).await;
    Ok(Json(users))
}

#[get_api("/user/{id}")]
async fn get_user_by_id(
    Component(userservice): Component<UserService>,
    Path(id): Path<i64>,
) -> Result<Json<UserDto>, StatusCode> {
    Ok(Json(userservice.get_user(id).await))
}

#[get_api("/sendemail")]
async fn sendemail(
    Component(mail_service): Component<MailService>,
) -> Result<Json<MailResponse>, StatusCode> {
    let resp = mail_service
        .send_mail("demo1@demo.com".to_string())
        .await
        .unwrap();

    Ok(Json(MailResponse {
        success: resp.code().to_string() == "200".to_string(),
        message: resp.message().collect(),
    }))
}

#[get("/user-info")]
async fn protected_user_info(
    claims: Claims,
    Config(conf): Config<CustomConfig>,
) -> impl IntoResponse {
    let user_id = claims.sub;
    format!("get user info of id#{}: {}", user_id, conf.user_info_detail)
}

#[post_api("/user")]
async fn create_user(Component(userservice): Component<UserService>,
    Json(payload): Json<UserInput>) -> Result<Json<UserResponse>, StatusCode> {
    let s = format!(
        "Received user: {} ({} years old)",
        payload.name, payload.firstname
    );
        let t = payload.validate();
        if t.is_err(){
            tracing::error!("Validation error: {:?}", t);
            Err(
            StatusCode::BAD_REQUEST
            )

        } else {
            
    let user = UserDto {
        id: None,
        name: payload.name, 
        firstname: payload.firstname,
        age: payload.age,
    };
    tracing::info!(s);
    let u1 = userservice.create_user(user).await;
    Ok(Json(UserResponse {
        id: u1.id.unwrap(),   

        message: "User data retrieved successfully.".to_string(),
    }))
}
}
