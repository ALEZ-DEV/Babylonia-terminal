use std::{path::PathBuf, sync::Arc};

use downloader::progress::Reporter;

pub trait ComponentDownloader {
    async fn install<P: Reporter + 'static>(&self, progress: Option<Arc<P>>) -> anyhow::Result<()>;

    //the 'static is something to change, I don't very like it, but it's for testing purpose
    async fn download<P: Reporter + 'static>(
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf>;

    async fn uncompress(file: PathBuf, new_filename: PathBuf) -> anyhow::Result<()>;
}
