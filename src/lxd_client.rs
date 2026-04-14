use anyhow::Result;
use http::{Request, StatusCode, Uri};
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::client::conn::{http1, http1::SendRequest};
use hyper::header::HOST;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::net::UnixStream;
use tracing::{debug, error, info};

use crate::lxd_api::{
    APIResult, ClusterMetadata, ErrorResult, ImageMetadata, LxdCommand, OkResult,
};

const LXD_BASE_URL: &str = "http://localhost/1.0";
const LXD_SOCKET: &str = "/var/snap/lxd/common/lxd-user/unix.socket";

#[derive(Debug, Clone)]
pub struct SocketClient {
    socket_path: PathBuf,
}

impl Default for SocketClient {
    fn default() -> Self {
        Self {
            socket_path: LXD_SOCKET.into(),
        }
    }
}

impl SocketClient {
    /// Run the handshake, `await` for the `connection` and return
    /// a `SendableClient`.
    #[tracing::instrument]
    pub async fn connect(self) -> Result<SendableClient> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        // Just a hyper wrapper over a tokio stream.
        let hyper_stream = hyper_util::rt::TokioIo::new(stream);

        let (sender, connection) = http1::handshake(hyper_stream).await?;
        tokio::task::spawn(async move {
            match connection.await {
                Ok(_) => info!("Connection to socket finished."),
                Err(err) => error!("Failed to connect to socket, with error messsage: {err:?}"),
            }
        });
        Ok(SendableClient::new(sender))
    }
}

#[derive(Debug)]
pub struct SendableClient {
    sender: SendRequest<Empty<Bytes>>,
}

impl SendableClient {
    fn new(sender: SendRequest<Empty<Bytes>>) -> Self {
        Self { sender }
    }

    /// Run `GET` requests.
    async fn make_get_request<B: DeserializeOwned + std::fmt::Debug>(
        &mut self,
        request: Request<Empty<Bytes>>,
    ) -> Result<APIResult<B>> {
        let response = self.sender.send_request(request).await?;

        match response.status() {
            StatusCode::OK => {
                let body_response = response.collect().await?.to_bytes();
                let ok: OkResult<B> = serde_json::from_slice(&body_response)?;
                Ok(APIResult::Ok(ok))
            }
            status => {
                error!("API returned error: {status}");
                let body_response = response.collect().await?.to_bytes();
                error!(
                    "Response body: {:?}",
                    String::from_utf8(body_response.to_vec())
                );
                let err: ErrorResult = serde_json::from_slice(&body_response)?;
                Ok(APIResult::Err(err))
            }
        }
    }
}

impl LxdCommand for SendableClient {
    #[tracing::instrument(fields(req_type = "images"))]
    /// Call the `/images` GET api.
    async fn list_images(&mut self) -> Result<APIResult<ImageMetadata>> {
        let uri = Uri::from_str(&format!("{}/images", LXD_BASE_URL))?;
        debug!("URI: {uri:?}");

        let authority = uri.authority().ok_or(anyhow::anyhow!("oi"))?.to_string();

        let request = Request::builder()
            .uri(uri)
            .header(HOST, authority)
            .method("GET")
            .body(Empty::<Bytes>::new())?;

        self.make_get_request(request).await
    }

    /// Call the `/cluster` GET api.
    #[tracing::instrument(fields(req_type = "clusters"))]
    async fn list_clusters(&mut self) -> Result<APIResult<ClusterMetadata>> {
        let uri = Uri::from_str(&format!("{}/cluster", LXD_BASE_URL))?;
        debug!("URI: {uri:?}");

        let authority = uri.authority().ok_or(anyhow::anyhow!("oi"))?.to_string();

        let request = Request::builder()
            .uri(uri)
            .header(HOST, authority)
            .method("GET")
            .body(Empty::<Bytes>::new())?;

        self.make_get_request(request).await
    }
}
