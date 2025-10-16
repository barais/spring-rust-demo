use spring::tracing;
use spring_web::extractor::Query;
use spring_web::{
    axum::Json, axum::http::StatusCode, axum::response::IntoResponse, extractor::Component,
    extractor::Path
};
use validator::Validate;

use crate::dto::maildto::MailResponse;
use crate::dto::userdto::{ UserDto, UserInput, UserResponse};
use crate::service::mailservice::MailService;
use crate::service::userservice::UserService;
use crate::web::jwt::{has_authorities, Claims};
use crate::web::pagination::Pagination;
use spring_web::{ get_api, post_api};

    use spring_web::{ error::KnownWebError};





#[get_api("/hello")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

#[get_api("/test")]
async fn hello_world1() -> impl IntoResponse {
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
    let user = userservice.get_user(id).await;

    if user.is_none(){
        return Err(StatusCode::NOT_FOUND);
    } else {
        Ok(Json(user.unwrap()))
    }
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

    #[get_api("/error")]
    async fn error_request() ->     spring_web::error::Result<String> {
        Err(KnownWebError::bad_request("request error"))?
    }

#[get_api("/user-info")]
async fn protected_user_info(
    claims: Claims
) -> impl IntoResponse {
    let s = has_authorities(&claims, "USER".to_string());
    println!("🔐 [AUTH] Checking authentication for: user_info : {}",s);    
    
    let user_id = claims.sub;
    format!("get user info of id#{}", user_id)
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
            
    let user = payload.into();
    
    tracing::info!(s);
    let u1 = userservice.create_user(user).await;
    Ok(Json(UserResponse {
        id: u1.id.unwrap(),   

        message: "User data retrieved successfully.".to_string(),
    }))
}
}

