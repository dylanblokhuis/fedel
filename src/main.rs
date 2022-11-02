use std::net::SocketAddr;

use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

mod components;
mod routes;
mod ssr;

#[tokio::main]
async fn main() {
    let mut app = Router::new().nest("/admin", routes::admin_router());

    if cfg!(debug_assertions) {
        app = app
            .route("/socket", get(ws_handler))
            .route("/hotreload.js", get(hot_reload_js));
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    std::env::set_var("PORT", port.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket) {
        if let Some(_msg) = socket.recv().await {}
    }

    ws.on_upgrade(handle_socket)
}

async fn hot_reload_js() -> impl IntoResponse {
    Response::builder()
        .header("content-type", "text/javascript")
        .body(include_str!("hotreload.js").to_string())
        .unwrap()
}
