use std::{env, net::SocketAddr};

use middle_mile::adapters::rest_api::routers::routers;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind(&env::var("SERVER_IP_PORT").unwrap_or("0.0.0.0:80".into()))
        .await
        .unwrap();

    println!("Server running...");
    axum::serve(
        listener,
        routers().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
