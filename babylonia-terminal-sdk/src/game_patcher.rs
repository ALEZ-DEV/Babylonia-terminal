use std::{io::Read, path::PathBuf};

use log::debug;
use rust_embed::RustEmbed;
use tokio::{
    fs::{remove_file, rename, File},
    io::AsyncWriteExt,
};

use crate::{
    game_state::{GameConfig, GameState},
    utils::{get_game_name, get_game_name_with_executable},
};

pub async fn patch_game(game_dir: PathBuf) -> anyhow::Result<()> {
    // section to fix bad named file

    debug!("{:?}", game_dir);

    let unity_resources = game_dir.join(get_game_name()).join(format!(
        "{}{}",
        get_game_name(),
        "_Data/Resources/unity%20default%20resources"
    ));

    debug!("unity resources exist? : {}", unity_resources.exists());
    if unity_resources.exists() {
        rename(
            unity_resources.clone(),
            unity_resources
                .parent()
                .unwrap()
                .join("unity default resources"),
        )
        .await?;
    } else if !unity_resources
        .parent()
        .unwrap()
        .join("unity default resources")
        .exists()
    {
        anyhow::bail!(
            "{} file doesn't exists ? Please restart the launcher and make a repair.",
            unity_resources.to_str().unwrap()
        );
    }

    // section to replace KRSDKExternal.exe by an empty one
    //
    let krsdk_external_path = game_dir.join(get_game_name()).join(format!(
        "{}{}",
        get_game_name(),
        "_Data/Plugins/KRSDKExternal.exe"
    ));

    remove_file(krsdk_external_path.clone()).await?;
    File::create(krsdk_external_path).await?;

    // section to replace the executable with the patched one

    let executable_path = game_dir
        .join(get_game_name())
        .join(get_game_name_with_executable());

    debug!("{:?}", executable_path);

    if executable_path.exists() {
        remove_file(executable_path.clone()).await?;
    }

    match PatchedGameExecutable::get_exectable() {
        Some(exe) => {
            let mut file = File::create(executable_path).await?;

            let data: Result<Vec<_>, _> = exe.data.bytes().collect();
            let data = data.expect("Unable to read executable data");

            file.write_all(&data).await?;
        }
        None => anyhow::bail!(
            "Game executable not included in the binary! Please report this to the developer!"
        ),
    }

    let mut config = GameConfig::get_config().await;
    config.is_game_patched = true;
    GameConfig::save_config(config).await?;

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct PatchedGameExecutable;

impl PatchedGameExecutable {
    fn get_exectable() -> Option<rust_embed::EmbeddedFile> {
        PatchedGameExecutable::get("patched.exe")
    }
}
