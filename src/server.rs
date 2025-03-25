use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use crate::system_info::SystemInfo;

pub async fn start_server(port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new()
        .route("/get_info", get(info));

    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn info() -> Json<SystemInfo> {
    Json(crate::system_info::get_info().await)
}