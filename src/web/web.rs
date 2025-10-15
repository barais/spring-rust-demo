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
use spring_web::{  get_api, post_api};

use spring_web::middlewares;
use spring_web::{
    axum::{
        body,
        middleware::{self, Next},
        response::{ Response},
    },
    extractor::Request,
};
use tower_http::timeout::TimeoutLayer;
        use std::time::Duration;

#[middlewares(
    middleware::from_fn(problem_middleware),
    TimeoutLayer::new(Duration::from_secs(10))
)]
mod web{
    use spring_web::error::KnownWebError;

    use super::*;

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

    #[get("/error")]
    async fn error_request() ->     spring_web::error::Result<String> {
        Err(KnownWebError::bad_request("request error"))?
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

/// ProblemDetail: https://www.rfc-editor.org/rfc/rfc7807
async fn problem_middleware(
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri().path().to_string();
    let response = next.run(request).await;
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        let bytes = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("server body read failed");
        let msg = String::from_utf8(bytes.to_vec()).expect("read body to string failed");

        // error log into db
        tracing::error!("{} {} {}", status.as_u16(), uri, msg);
        problemdetails::new(status)
            .with_instance(uri)
            .with_title(status.canonical_reason().unwrap_or("error"))
            .with_detail(msg)
            .into_response()
    } else {
        response
    }
}

}
