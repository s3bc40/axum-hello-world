use std::net::SocketAddr;

use axum::{extract::ConnectInfo, response::Html, routing::get};
use tokio::net::TcpListener;

use backend_ai_agent::alloy_test;
use backend_ai_agent::hello_world;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // test the alloy provider
    let _ = alloy_test::test_alloy().await;

    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let router = hello_world::create_router().route("/", get(index));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

// handler function that returns a simple string
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<h1>Your ip address is: \"{addr}\"</h1>\n\
         <h2>You are in immediate danger of getting identified by bad people.</h2>\n\
         <h2>Thankfully we have a VPN service to hide your ip.</h2>\n\
         <h2>Visit <a href=\"http://localhost:3000/average_joe_absolutely_needs_vpn\">THIS</a> link to download it.</h2>"
    );

    Html(html)
}
