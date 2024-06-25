use std::{env, fs, path::Path};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

const SERVER_PATH: &str = "node_modules/@biomejs/biome/bin/biome";
const PACKAGE_NAME: &str = "@biomejs/biome";

struct BiomeExtension;

impl BiomeExtension {
  fn server_exists(&self, path: &str) -> bool {
    fs::metadata(path).map_or(false, |stat| stat.is_file())
  }

  fn server_script_path(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<String> {
    // This is a workaround, as reading the file from wasm doesn't work.
    // Instead we try to read the `package.json`, see if `@biomejs/biome` is installed
    let package_json = worktree
      .read_text_file("package.json")
      .unwrap_or(String::from(r#"{}"#));
    let package_json: Option<serde_json::Value> = serde_json::from_str(package_json.as_str()).ok();

    let server_package_exists = package_json.is_some_and(|f| {
      !f["dependencies"]["@biomejs/biome"].is_null()
        || !f["devDependencies"]["@biomejs/biome"].is_null()
    });

    if server_package_exists {
      let worktree_root_path = worktree.root_path();
      let worktree_server_path = Path::new(worktree_root_path.as_str())
        .join(SERVER_PATH)
        .to_string_lossy()
        .to_string();

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
    Self
  }

  fn language_server_command(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<zed::Command> {
    let path = self.server_script_path(language_server_id, worktree)?;
    let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

    let mut args = vec![
      env::current_dir()
        .unwrap()
        .join(path)
        .to_string_lossy()
        .to_string(),
      "lsp-proxy".to_string(),
    ];

    if let Some(settings) = settings.settings {
      let config_path = settings.get("config_path").and_then(|value| value.as_str());

      if let Some(path) = config_path {
        args.push("--config-path".to_string());
        args.push(path.to_string());
      }
    }

    if let Some(binary) = settings.binary {
      return Ok(zed::Command {
        command: binary.path.map_or(zed::node_binary_path()?, |path| path),
        args: binary.arguments.map_or(args, |args| args),
        env: Default::default(),
      });
    }

    Ok(zed::Command {
      command: zed::node_binary_path()?,
      args,
      env: Default::default(),
    })
  }
}

zed::register_extension!(BiomeExtension);
