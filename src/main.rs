use std::net::SocketAddr;

use axum::{Router, response::Redirect, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .fallback(anything_else);

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// handler function that returns a simple string
async fn hello_world() -> &'static str {
    "Hello, world!"
}

// handler function for fallback
async fn anything_else() -> Redirect {
    Redirect::to("/hello-world")
}
