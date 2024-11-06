use std::{
    path::PathBuf,
    task::{Context, Poll},
};

use log::debug;
use tokio::fs;

pub async fn move_all_file_in_dir(dir: &PathBuf, new_dir: &PathBuf) -> Result<(), String> {
    if !dir.is_dir() {
        return Err(format!(
            "{} is not a directory",
            dir.as_os_str().to_str().unwrap()
        ));
    }

    if !dir.exists() {
        return Err(format!(
            "{} does not exist",
            dir.as_os_str().to_str().unwrap()
        ));
    }

    if let Ok(mut objects) = fs::read_dir(dir).await {
        while let Ok(Some(entry)) = objects.next_entry().await {
            debug!("{:?}", entry);
        }
    } else {
        return Err(format!(
            "Failed to read directory -> {}",
            dir.as_os_str().to_str().unwrap()
        ));
    }

    Ok(())
}
