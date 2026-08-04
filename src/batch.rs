use std::env;
use std::path::PathBuf;
use zed_extension_api::{self as zed, Result};

const SERVER_NAME: &str = "batch-language-server";

struct BatchExtension;

impl zed::Extension for BatchExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let script_path = env::current_dir()
            .map_err(|e| format!("failed to resolve work directory: {e}"))?
            .join(PathBuf::from(SERVER_NAME).join("server.js"));

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
