use std::borrow::{Borrow, BorrowMut};

use reqwest::header::{self, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRelease {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: String,
    pub assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

pub struct ComponentsDownloader;

pub trait GithubRequester {
    fn get_client() -> reqwest::Client {
        let mut headers = None;

        if let Ok(github_token) = std::env::var("BT_GITHUB_TOKEN") {
            headers = Some(header::HeaderMap::new());
            headers.as_mut().unwrap().insert(
                "Authorization",
                header::HeaderValue::from_str(&format!("Bearer {}", github_token)).unwrap(),
            );
        }

        let mut client = reqwest::Client::builder();

        if let Some(h) = headers {
            client = client.default_headers(h);
        }

        client
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build https client")
    }

    async fn get_latest_github_release(
        user: &str,
        repo_name: &str,
    ) -> anyhow::Result<Vec<GithubRelease>> {
        let response = Self::get_client()
            .get(format!(
                "https://api.github.com/repos/{}/{}/releases",
                user, repo_name
            ))
            .send()
            .await?;
        let body = response.text().await?;

        let releases: Vec<GithubRelease> = serde_json::from_str(&body)?;
        Ok(releases)
    }
}
