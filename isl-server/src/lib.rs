use axum::{routing::get, serve::Serve, Json, Router};
pub struct Bob {
    pub hello: String,
}

pub async fn setup_server() -> Serve<Router, Router> {
    let app: Router = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app)
}

async fn root() -> &'static str {
    "hello"
}
