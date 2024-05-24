use babylonia_terminal_sdk::{
    components::proton_component::{PROTON_DEV, PROTON_REPO},
    utils::github_requester::{GithubRelease, GithubRequester},
};

use crate::messages::github::{AskProtonVersions, ProtonVersions};

#[warn(dead_code)]
struct GithubInfo;

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
            Err(_) => todo!(),
        }
    }
}
