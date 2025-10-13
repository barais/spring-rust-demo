mod config;
mod dao;
mod domain;
mod dto;
mod service;
mod web;
mod welds;
use spring::{App, auto_config};
use spring_mail::MailPlugin;
use spring_web::{WebConfigurator, WebPlugin};

use crate::web::web::ApiDoc;
use crate::welds::welds::WeldsPlugin;

use spring_utoipa::UtoipaConfigurator;
use spring_utoipa::UtoipaPlugin;
use spring_utoipa::utoipa::OpenApi;

// Main function entry
#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WeldsPlugin)
        .add_plugin(WebPlugin)
        .add_plugin(MailPlugin)
        .add_plugin(UtoipaPlugin)
        .with_openapi(ApiDoc::openapi())
        .run()
        .await
}
