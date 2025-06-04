use std::path::{Path, PathBuf};
use zed::settings::LspSettings;
use zed_extension_api::{
  self as zed, LanguageServerId, Result, Worktree,
  serde_json::{self, Value},
};

const WORKTREE_SERVER_PATH: &str = "node_modules/@biomejs/biome/bin/biome";
const PACKAGE_NAME: &str = "@biomejs/biome";

const BIOME_CONFIG_PATHS: &[&str] = &["biome.json", "biome.jsonc"];

struct BiomeExtension;

impl BiomeExtension {
  fn extension_server_exists(&self, path: &PathBuf) -> bool {
    std::fs::metadata(path).is_ok_and(|stat| stat.is_file())
  }

  fn binary_specifier(&self) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();

    let binary_name = match platform {
      zed::Os::Windows => "biome.exe",
      _ => "biome",
    };

    Ok(format!(
      "@biomejs/cli-{platform}-{arch}/{binary}",
      platform = match platform {
        zed::Os::Mac => "darwin",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "win32",
      },
      arch = match arch {
        zed::Architecture::Aarch64 => "arm64",
        zed::Architecture::X8664 => "x64",
        _ => return Err(format!("unsupported architecture: {arch:?}")),
      },
      binary = binary_name,
    ))
  }

  fn worktree_biome_exists(&self, worktree: &zed::Worktree) -> bool {
    // This is a workaround, as reading the file from wasm doesn't work.
    // Instead we try to read the `package.json`, see if `@biomejs/biome` is installed
    let package_json = worktree
      .read_text_file("package.json")
      .unwrap_or(String::from(r#"{}"#));

    let package_json: Option<serde_json::Value> = serde_json::from_str(package_json.as_str()).ok();

    package_json.is_some_and(|f| {
      !f["dependencies"][PACKAGE_NAME].is_null() || !f["devDependencies"][PACKAGE_NAME].is_null()
    })
  }

  fn check_biome_updates(&mut self, language_server_id: &LanguageServerId) -> Result<()> {
    // fallback to extension owned biome
    zed::set_language_server_installation_status(
      language_server_id,
      &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );

    let extension_server_path = &Path::new("./node_modules").join(self.binary_specifier()?);
    let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

    if !self.extension_server_exists(extension_server_path)
      || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
    {
      zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
      );
      let result = zed::npm_install_package(PACKAGE_NAME, &version);
      match result {
        Ok(()) => {
          if !self.extension_server_exists(extension_server_path) {
            Err(format!(
              "installed package '{PACKAGE_NAME}' did not contain expected path '{extension_server_path:?}'",
            ))?;
          }
        }
        Err(error) => {
          if !self.extension_server_exists(extension_server_path) {
            Err(format!(
              "failed to install package '{PACKAGE_NAME}': {error}"
            ))?;
          }
        }
      }
    }

    Ok(())
  }

  // Returns the path if a config file exists
  pub fn config_path(&self, worktree: &zed::Worktree, settings: &Value) -> Option<String> {
    let config_path_setting = settings.get("config_path").and_then(|value| value.as_str());

    if let Some(config_path) = config_path_setting {
      if worktree.read_text_file(config_path).is_ok() {
        return Some(config_path.to_string());
      } else {
        return None;
      }
    }

    for config_path in BIOME_CONFIG_PATHS {
      if worktree.read_text_file(config_path).is_ok() {
        return Some(config_path.to_string());
      }
    }

    None
  }

  fn require_config_file(&self, settings: &Value) -> bool {
    settings
      .get("require_config_file")
      .and_then(|value| value.as_bool())
      .unwrap_or(false)
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
    let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

    let mut args = vec!["lsp-proxy".to_string()];

    // check and run biome with custom binary
    if let Some(binary) = settings.binary {
      return Ok(zed::Command {
        command: binary
          .path
          .map_or(WORKTREE_SERVER_PATH.to_string(), |path| path),
        args: binary.arguments.map_or(args, |args| args),
        env: Default::default(),
      });
    }

    // try to run from worktree biome package
    if self.worktree_biome_exists(worktree) {
      let server_path = Path::new(worktree.root_path().as_str())
        .join(WORKTREE_SERVER_PATH)
        .to_string_lossy()
        .to_string();

      let mut node_args = vec![server_path];
      node_args.append(&mut args);

      return Ok(zed::Command {
        command: zed::node_binary_path()?,
        args: node_args,
        env: Default::default(),
      });
    }

    // install/update and run biome for extension
    self.check_biome_updates(language_server_id)?;

    let mut server_path = PathBuf::from("./node_modules");
    server_path.push(self.binary_specifier()?);

    Ok(zed::Command {
      command: server_path.to_string_lossy().to_string(),
      args,
      env: Default::default(),
    })
  }

  fn language_server_workspace_configuration(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &Worktree,
  ) -> Result<Option<Value>> {
    let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

    let Some(settings) = lsp_settings.settings else {
      return Ok(Some(serde_json::json!({
        "biome": {},
      })));
    };

    let config_path = self
      .config_path(worktree, &settings)
      .map(|p| Path::new(&worktree.root_path()).join(p));

    Ok(Some(serde_json::json!({
      "biome": {
        "requireConfiguration": self.require_config_file(&settings),
        "configurationPath": config_path,
      },
    })))
  }
}

zed::register_extension!(BiomeExtension);
