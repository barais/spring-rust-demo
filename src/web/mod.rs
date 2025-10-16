pub mod pagination;
pub mod jwt;



pub mod web;

use spring_web::{
    axum::{
        body,
        middleware::{self, Next},
        response::{IntoResponse, Response},
    },
    extractor::{ Request},
    Router,
};

pub fn router() -> Router {
    Router::new().nest(
        "/api",
        spring_web::handler::auto_router()
            .layer(middleware::from_fn(problem_middleware))
      //      .layer(middleware::from_fn(auth_middleware)),
    )
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
/* async fn auth_middleware(
    request: Request,
    next: Next,
) -> Response {
    println!("🔐 [AUTH] Checking authentication for: {}", request.uri().path());
    
    

    if request.headers().get("Authorization").is_none() {
        return Response::builder()
            .status(401)
            .body("Unauthorized".into())
            .unwrap();
    } 

    next.run(request).await
}
*/
