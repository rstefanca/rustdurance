mod app;
mod controller;
mod dto;
mod error;
mod event;
mod event_bus;
mod listener;
mod service;
mod state;
pub mod plugin;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    println!("listening on {addr}");

    let state = state::AppState::new();
    listener::spawn_all(&state.event_bus);

    axum::serve(listener, app::router_with(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
