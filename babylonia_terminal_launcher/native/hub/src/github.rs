use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{DXVK_DEV, DXVK_REPO},
        proton_component::{PROTON_DEV, PROTON_REPO},
    },
    utils::github_requester::{GithubRelease, GithubRequester},
};

use crate::messages::{
    error::ReportError,
    github::{AskDxvkVersions, AskProtonVersions, DxvkVersions, ProtonVersions},
};

#[warn(dead_code)]
pub struct GithubInfo;

impl GithubRequester for GithubInfo {
    fn set_github_release_index(&mut self, _: usize) {
        todo!()
    }
}

pub async fn listen_proton_version() {
    let mut receiver = AskProtonVersions::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let releases: Result<Vec<GithubRelease>, _> =
            GithubInfo::get_github_releases(PROTON_DEV, PROTON_REPO).await;
        match releases {
            Ok(r) => ProtonVersions {
                versions: r.iter().map(|v| v.tag_name.to_owned()).collect(),
            }
            .send_signal_to_dart(),
            Err(e) => ReportError {
                error_message: format!("When fetching proton versions : {}", e),
            }
            .send_signal_to_dart(),
        }
    }
}

pub async fn listen_dxvk_version() {
    let mut receiver = AskDxvkVersions::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let releases: Result<Vec<GithubRelease>, _> =
            GithubInfo::get_github_releases(DXVK_DEV, DXVK_REPO).await;
        match releases {
            Ok(r) => DxvkVersions {
                versions: r.iter().map(|v| v.tag_name.to_owned()).collect(),
            }
            .send_signal_to_dart(),
            Err(e) => ReportError {
                error_message: format!("When fetching dxvk versions : {}", e),
            }
            .send_signal_to_dart(),
        }
    }
}
