use std::{path::PathBuf, sync::Arc};

use downloader::progress::Reporter;
use downloader::Downloader as FileDownloader;
use reqwest::header::{self, USER_AGENT};
use serde::{Deserialize, Serialize};
use tar::Header;
//use tokio::{fs::File, io};

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

static GITHUB_TOKEN: &'static str = "Bearer <GITHUB_TOKEN>"; //this token can only read public repo

impl ComponentsDownloader {
    fn get_client() -> reqwest::Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_static(GITHUB_TOKEN),
        );

        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Failed to build https client")
    }

    async fn get_latest_github_release(
        user: &str,
        repo_name: &str,
    ) -> anyhow::Result<Vec<GithubRelease>> {
        let response = ComponentsDownloader::get_client()
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

    pub async fn download_latest_wine<P>(
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf>
    where
        P: Reporter + 'static, //the 'static is something to change, I just made it like this for test prupose
    {
        let releases =
            //ComponentsDownloader::get_latest_github_release("Kron4ek", "Wine-Builds").await?;
            ComponentsDownloader::get_latest_github_release("GloriousEggroll", "wine-ge-custom").await?;

        let asset = releases[0]
            .assets
            .get(1)
            .expect("Asset not found in the github release");

        let mut downloader = FileDownloader::builder()
            .download_folder(output_dir)
            .parallel_requests(1)
            .build()?;

        let mut dl = downloader::Download::new(&asset.browser_download_url);
        if let Some(p) = progress {
            dl = dl.progress(p);
        }

        let _result = downloader.async_download(&[dl]).await?;
        let file_location = output_dir.join(asset.name.clone());

        Ok(file_location)
    }

    pub async fn download_latest_dxvk<P>(
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf>
    where
        P: Reporter + 'static,
    {
        let releases = ComponentsDownloader::get_latest_github_release("doitsujin", "dxvk").await?;

        let asset = releases[0]
            .assets
            .first()
            .expect("Asset not found in the github release");

        let mut downloader = FileDownloader::builder()
            .download_folder(output_dir)
            .parallel_requests(1)
            .build()?;
        let mut dl = downloader::Download::new(&asset.browser_download_url);
        if let Some(p) = progress {
            dl = dl.progress(p);
        }

        let _result = downloader.async_download(&[dl]).await?;
        let file_location = output_dir.join(asset.name.clone());

        Ok(file_location)
    }
}
