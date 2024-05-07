use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use tokio::fs::read_to_string;
use tokio::io::AsyncWriteExt;

use crate::game_state::GameState;

// start data ---------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub default: Default,
    pub hash_cache_check_acc_switch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub cdn_list: Vec<CdnList>,
    pub changelog: Changelog,
    pub resources: String,
    pub resources_base_path: String,
    pub resources_diff: ResourcesDiff,
    pub resources_exclude_path: Vec<String>,
    pub resources_exclude_path_need_update: Vec<String>,
    pub sample_hash_switch: i64,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnList {
    #[serde(rename = "K1")]
    pub k1: i64,
    #[serde(rename = "K2")]
    pub k2: i64,
    #[serde(rename = "P")]
    pub p: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changelog {
    #[serde(rename = "zh-Hans")]
    pub zh_hans: String,
    pub en: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesDiff {
    pub current_game_info: CurrentGameInfo,
    pub previous_game_info: PreviousGameInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfo {
    pub file_name: String,
    pub md5: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousGameInfo {
    pub file_name: String,
    pub md5: String,
    pub version: String,
}

// Resources ---------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub resource: Vec<Resource>,
    pub sample_hash_info: SampleHashInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub dest: String,
    pub md5: String,
    pub sample_hash: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleHashInfo {
    pub sample_num: i64,
    pub sample_block_max_size: i64,
}

// end data ---------------------------------------------------------------------

static URL: &str = concat!(
    "https://prod-alicdn-gamestarter.k",
    "uro",
    "gam",
    "e.com/pcstarter/prod/game/G143/4/index.json"
);

impl GameInfo {
    pub async fn get_info() -> anyhow::Result<GameInfo> {
        let info = match GameInfo::try_load_from_cache().await {
            Ok(i) => i,
            Err(_) => {
                let i = GameInfo::fetch_game_info().await?;
                i.save_in_cache().await?;
                i
            }
        };

        Ok(info)
    }

    pub async fn need_update(&self) -> anyhow::Result<bool> {
        let info = GameInfo::fetch_game_info().await?;
        Ok(self.default.version != info.default.version)
    }

    pub fn get_first_cdn(&self) -> String {
        self.default.cdn_list.first().unwrap().url.clone()
    }

    pub fn get_resource_base_path(&self) -> String {
        self.default.resources_base_path.clone()
    }

    pub async fn fetch_resources(&self) -> anyhow::Result<Resources> {
        let resources_base_url = self.get_first_cdn();
        let resources_path_url = &self.default.resources;
        let resources_url = format!("{}{}", resources_base_url, resources_path_url);

        let response = reqwest::get(&resources_url).await?;
        let body = response.text().await?;
        Ok(serde_json::from_str::<Resources>(&body)?)
    }

    async fn fetch_game_info() -> anyhow::Result<GameInfo> {
        let response = reqwest::get(URL).await?;
        let body = response.text().await?;
        Ok(serde_json::from_str(&body)?)
    }

    async fn get_cache_file_path() -> PathBuf {
        GameState::get_config_directory()
            .await
            .join("version-cache")
    }

    async fn save_in_cache(&self) -> anyhow::Result<()> {
        let _ = tokio::fs::create_dir(GameState::get_config_directory().await).await;
        let mut file = tokio::fs::File::create(GameInfo::get_cache_file_path().await).await?;

        let content = serde_json::to_string(self)?;

        file.write_all(content.as_bytes()).await?;

        Ok(())
    }

    async fn try_load_from_cache() -> anyhow::Result<Self> {
        let content = read_to_string(GameInfo::get_cache_file_path().await).await?;
        Ok(serde_json::from_str::<GameInfo>(&content)?)
    }
}

impl Resources {
    pub fn get_max_size_resources(&self) -> u64 {
        let mut max_size: u64 = 0;

        self.resource.iter().for_each(|r| max_size += r.size as u64);

        max_size
    }
}

impl Resource {
    pub fn build_download_url(&self, base_url: &str, zip_uri: &str) -> String {
        format!("{}{}{}", base_url, zip_uri, self.dest)
    }
}
