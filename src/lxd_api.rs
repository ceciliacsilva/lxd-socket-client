#![allow(dead_code)]
use anyhow::Result;
use serde::Deserialize;

pub trait LxdCommand {
    async fn list_images(&mut self) -> Result<APIResult<ImageMetadata>>;

    async fn list_clusters(&mut self) -> Result<APIResult<ClusterMetadata>>;
}

#[derive(Debug)]
pub enum APIResult<T> {
    Ok(OkResult<T>),
    Err(ErrorResult),
}

#[derive(Debug, Deserialize)]
pub struct OkResult<T> {
    metadata: T,
    status: String,
    status_code: u32,
    #[serde(rename = "type")]
    rtype: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResult {
    error: String,
    status_code: u32,
    #[serde(rename = "type")]
    rtype: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageMetadata {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    metadata: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClusterMetadata {
    enabled: bool,
    member_config: Vec<MemberConfig>,
    server_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MemberConfig {
    entity: String,
    name: String,
    key: String,
    value: String,
    description: String,
}
