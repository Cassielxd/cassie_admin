use axum::{response::IntoResponse, routing::get, Router, Server};

use crate::{config::ApplicationConfig, APPLICATION_CONTEXT};
use std::time::Duration;
use log::info;
use tower_http::cors::{Any, CorsLayer};

//server 测试
async fn index() -> impl IntoResponse {
    let msg = format!("hello");
    msg
}

//初始化一个本地server
pub fn init_server() {
    tokio::spawn(async {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        let server = format!("{}:{}", cassie_config.server().host(), cassie_config.server().port());
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any).max_age(Duration::from_secs(60) * 10);
        let app = Router::new().route("/", get(index)).layer(cors);
        info!("local://{}",server);
        Server::bind(&server.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    });
}
