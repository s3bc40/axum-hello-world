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

pub mod alloy_test {
    use alloy::providers::{Provider, ProviderBuilder};
    use eyre::Result;

    pub async fn test_alloy() -> Result<()> {
        // Set up the HTTP transport which is consumed by the RPC client.
        let rpc_url = "https://eth.merkle.io".parse()?;

        // Create a provider using the HTTP transport.
        let provider = ProviderBuilder::new().on_http(rpc_url);

        // Get latest block number.
        let latest_block = provider.get_block_number().await?;

        // Print the block number.
        println!("Latest block number: {latest_block}");

        Ok(())
    }
}
