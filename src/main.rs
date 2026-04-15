mod lxd_api;
mod lxd_client;

use crate::lxd_api::LxdCommand;
use crate::lxd_client::SocketClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Just a simple test:
    //  - one task listing images and cluster;
    //  - another one listeing cluster.
    let h1 = tokio::spawn(async move {
        let mut sock = SocketClient::default().connect().await.unwrap();
        let images = sock.list_images().await;
        let clusters = sock.list_clusters().await;
        println!("Images: {images:?} and clusters: {clusters:?}");
    });

    let h2 = tokio::spawn(async move {
        let mut sock = SocketClient::default().connect().await.unwrap();
        let clusters = sock.list_clusters().await;
        println!("clusters: {clusters:?}");
    });

    let _ = tokio::join!(h1, h2);
}
