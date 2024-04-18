use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use super::component_downloader::ComponentDownloader;
use crate::utils::github_requester::GithubRequester;

pub struct PGRComponent {
    path: PathBuf,
}

impl GithubRequester for PGRComponent {}

impl ComponentDownloader for PGRComponent {
    async fn install<P: downloader::progress::Reporter + 'static>(
        &self,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<()> {
        Self::download(self.path, progress);
    }

    async fn download<P: downloader::progress::Reporter + 'static>(
        output_dir: &std::path::PathBuf,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<std::path::PathBuf> {
    }

    async fn uncompress(
        file: std::path::PathBuf,
        new_filename: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
