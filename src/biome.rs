use std::{env, fs, path::Path};
use zed_extension_api::{self as zed, LanguageServerId, Result};

const SERVER_PATH: &str = "node_modules/@biomejs/biome/bin/biome";
const PACKAGE_NAME: &str = "@biomejs/biome";

struct BiomeExtension {}

impl BiomeExtension {
  fn server_exists(&self, path: &str) -> bool {
    fs::metadata(path).map_or(false, |stat| stat.is_file())
  }

  fn server_script_path(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<String> {
    let worktree_root_path = worktree.root_path();
    let worktree_server_path = Path::new(worktree_root_path.as_str())
      .join(SERVER_PATH)
      .to_string_lossy()
      .to_string();

    // This is a workaround, as reading the file from wasm doesn't work.
    // Instead we try to read the `@biomejs/biome`, see if the package exists
    let worktree_server_exists = worktree
      .read_text_file("node_modules/@biomejs/biome/package.json")
      .map_or(false, |_f| true);
    if worktree_server_exists {
      return Ok(worktree_server_path);
    }

    zed::set_language_server_installation_status(
      language_server_id,
      &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );

    let fallback_server_path = SERVER_PATH.to_string();
    let fallback_server_exist = self.server_exists(fallback_server_path.as_str());
    let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

    if !fallback_server_exist
      || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
    {
      zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
      );
      let result = zed::npm_install_package(PACKAGE_NAME, &version);
      match result {
        Ok(()) => {
          if !self.server_exists(fallback_server_path.as_str()) {
            Err(format!(
              "installed package '{PACKAGE_NAME}' did not contain expected path '{fallback_server_path}'",
            ))?;
          }
        }
        Err(error) => {
          if !self.server_exists(fallback_server_path.as_str()) {
            Err(error)?;
          }
        }
      }
    }

    Ok(fallback_server_path.to_string())
  }
}

impl zed::Extension for BiomeExtension {
  fn new() -> Self {
    Self {}
  }

  fn language_server_command(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<zed::Command> {
    let path = self.server_script_path(language_server_id, worktree)?;

    Ok(zed::Command {
      command: zed::node_binary_path()?,
      args: vec![
        env::current_dir()
          .unwrap()
          .join(path)
          .to_string_lossy()
          .to_string(),
        "lsp-proxy".to_string(),
      ],
      env: Default::default(),
    })
  }
}

zed::register_extension!(BiomeExtension);
