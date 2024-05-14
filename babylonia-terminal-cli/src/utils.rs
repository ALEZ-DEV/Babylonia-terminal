use babylonia_terminal_sdk::utils::github_requester::GithubRequester;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn use_latest(prompt: &str) -> bool {
    let choices = ["Install latest", "Choose specific version"];

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();

    if choices[index] == "Install latest" {
        true
    } else {
        false
    }
}

struct Chooser;
impl GithubRequester for Chooser {
    fn set_github_release_index(&mut self, _: usize) {}
}

pub async fn choose_release_version(
    github_username: &str,
    repo_name: &str,
    prompt: &str,
) -> anyhow::Result<usize> {
    let releases = Chooser::get_github_releases(github_username, repo_name).await?;
    let releases_names: Vec<String> = releases.iter().map(|r| r.tag_name.to_owned()).collect();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&releases_names)
        .max_length(5)
        .interact()
        .unwrap();

    Ok(index)
}
