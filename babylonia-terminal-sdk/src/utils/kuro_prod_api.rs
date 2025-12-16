use std::path::PathBuf;

use log::debug;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tokio::fs::read_to_string;
use tokio::io::AsyncWriteExt;

use crate::game_config::GameConfig;

// start data ---------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub chunk_download_switch: i64,
    pub key_file_check_switch: i64,
    pub resources_login: ResourcesLogin,
    pub check_exe_is_running: i64,
    pub hash_cache_check_acc_switch: i64,
    pub fingerprints: Vec<String>,
    pub default: Default,
    #[serde(rename = "RHIOptionSwitch")]
    pub rhioption_switch: i64,
    pub predownload_switch: i64,
    #[serde(rename = "RHIOptionList")]
    pub rhioption_list: Vec<Value>,
    pub experiment: Experiment,
    pub key_file_check_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesLogin {
    pub host: String,
    pub login_switch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub cdn_list: Vec<CdnList>,
    pub changelog: Changelog,
    pub changelog_visible: i64,
    pub config: Config,
    pub resources: String,
    pub resources_base_path: String,
    pub resources_diff: ResourcesDiff,
    pub resources_exclude_path: Vec<String>,
    pub resources_exclude_path_need_update: Vec<String>,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnList {
    #[serde(rename = "P")]
    pub p: i64,
    #[serde(rename = "K1")]
    pub k1: i64,
    #[serde(rename = "K2")]
    pub k2: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changelog {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub index_file_md5: String,
    pub un_compress_size: i64,
    pub base_url: String,
    pub size: i64,
    pub patch_type: String,
    pub zip_config: ZipConfig,
    pub index_file: String,
    pub resources_exclude_path_need_update: Vec<String>,
    pub version: String,
    pub resources_exclude_path: Vec<String>,
    pub patch_config: Vec<PatchConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipConfig {
    pub index_file_md5: String,
    pub un_compress_size: i64,
    pub ext: Ext,
    pub base_url: String,
    pub size: i64,
    pub index_file: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ext {
    pub max_file_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchConfig {
    pub index_file_md5: String,
    pub un_compress_size: i64,
    pub ext: Ext2,
    pub base_url: String,
    pub size: i64,
    pub index_file: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ext2 {
    pub max_file_size: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experiment {
    pub download: Download,
    #[serde(rename = "res_check")]
    pub res_check: ResCheck,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub download_cdn_select_test_duration: String,
    pub download_read_block_timeout: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResCheck {
    pub file_chunk_check_switch: String,
    pub file_size_check_switch: String,
    pub res_valid_check_time_out: String,
    pub file_check_white_list_config: String,
}

// Resources ---------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub resource: Vec<Resource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub dest: String,
    pub md5: String,
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
    "e.com/launcher/game/G143/50015_LWdk9D2Ep9mpJmqBZZkcPBU2YNraEWBQ/index.json"
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
        // I don't really like this if statement, but I will try to change it in the future, this is just for a quick fix
        if self.default.version != info.default.version {
            info.save_in_cache().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_first_cdn(&self) -> String {
        self.default.cdn_list.first().unwrap().url.clone()
    }

    pub fn get_resource_base_path(&self) -> String {
        let mut result = self.default.resources_base_path.clone();
        result.push('/');
        result
    }

    pub async fn fetch_resources(&self) -> anyhow::Result<Resources> {
        let resources_base_url = self.get_first_cdn();
        let resources_path_url = &self.default.resources;
        let resources_url = format!("{}{}", resources_base_url, resources_path_url);
        debug!("{}", &resources_url);

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
        GameConfig::get_config_directory()
            .await
            .join("version-cache")
    }

    async fn save_in_cache(&self) -> anyhow::Result<()> {
        let _ = tokio::fs::create_dir(GameConfig::get_config_directory().await).await;
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
