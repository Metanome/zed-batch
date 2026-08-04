use std::env;
use std::fs;
use std::path::PathBuf;
use zed_extension_api::{self as zed, Result};

const SERVER_NAME: &str = "batch-language-server";
const SERVER_REPO: &str = "PleahMaCaka/batch-language-server";
const SERVER_ASSET: &str = "server.js";

struct BatchExtension {
    cached_script: Option<PathBuf>,
}

impl BatchExtension {
    fn server_script(&mut self, language_server_id: &zed::LanguageServerId) -> Result<PathBuf> {
        if let Some(script) = &self.cached_script {
            if script.exists() {
                return Ok(script.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            SERVER_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("failed to fetch the latest language server release: {e}"))?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == SERVER_ASSET)
            .ok_or_else(|| format!("release {} has no {SERVER_ASSET}", release.version))?;

        let version_dir = format!("{SERVER_NAME}-{}", release.version);
        let script = PathBuf::from(&version_dir).join(SERVER_ASSET);

        if !script.exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            fs::create_dir_all(&version_dir)
                .map_err(|e| format!("failed to create {version_dir}: {e}"))?;

            let downloaded = zed::download_file(
                &asset.download_url,
                script.to_str().ok_or("language server path is not utf-8")?,
                zed::DownloadedFileType::Uncompressed,
            );

            if let Err(e) = downloaded {
                fs::remove_dir_all(&version_dir).ok();

                return Err(format!("failed to download the language server: {e}"));
            }

            remove_other_versions(&version_dir);
        }

        self.cached_script = Some(script.clone());

        Ok(script)
    }
}

fn remove_other_versions(version_dir: &str) {
    let Ok(entries) = fs::read_dir(".") else {
        return;
    };

    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy() != version_dir {
            fs::remove_dir_all(entry.path()).ok();
        }
    }
}

impl zed::Extension for BatchExtension {
    fn new() -> Self {
        Self {
            cached_script: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let script = self.server_script(language_server_id)?;
        let script_path = env::current_dir()
            .map_err(|e| format!("failed to resolve work directory: {e}"))?
            .join(script);

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                script_path.to_string_lossy().to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(BatchExtension);
