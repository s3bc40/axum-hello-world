pub mod hello_world {
    use axum::{Router, response::Redirect, routing::get};

    // handler function that returns a simple string
    pub async fn hello_world() -> &'static str {
        "Hello, world!"
    }

    // handler function for fallback
    pub async fn anything_else() -> Redirect {
        Redirect::to("/hello-world")
    }

    pub fn create_router() -> Router {
        Router::new()
            .route("/hello-world", get(hello_world))
            .fallback(anything_else)
    }
}
