use serde::Deserialize;
use serde::Serialize;

// start data ---------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
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
    "https://",
    "prod",
    "-alicdn",
    "-gamestarter",
    ".kur",
    "ogame",
    ".com/pcst",
    "arter/pro",
    "d/game/G1",
    "43/4/index.json"
);

pub async fn fetch_resources() -> anyhow::Result<Resources> {
    let response = reqwest::get(URL).await?;
    let body = response.text().await?;
    let index: Index = serde_json::from_str(&body)?;

    let resources_base_url = index.default.cdn_list.first().unwrap().url.clone();
    let resources_path_url = index.default.resources;
    let resources_url = format!("{}{}", resources_base_url, resources_path_url);

    let response = reqwest::get(&resources_url).await?;
    let body = response.text().await?;
    Ok(serde_json::from_str::<Resources>(&body)?)
}
