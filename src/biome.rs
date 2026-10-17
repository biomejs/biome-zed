use std::path::{Path, PathBuf};
use zed::settings::LspSettings;
use zed_extension_api::serde_json::Map;
use zed_extension_api::{
  self as zed, LanguageServerId, Result, Worktree,
  serde_json::{self, Value},
};

const WORKTREE_SERVER_PATH: &str = "node_modules/@biomejs/biome/bin/biome";
const EXTENSION_NODE_MODULES_PATH: &str = "./node_modules";
const PACKAGE_NAME: &str = "@biomejs/biome";
const BIOME_CLI_PACKAGE_PREFIX: &str = "cli-";

const BIOME_CONFIG_PATHS: &[&str] = &["biome.json", "biome.jsonc"];

struct BiomeExtension;

fn extension_server_path_from_node_modules(node_modules_path: &Path) -> Option<PathBuf> {
  let biome_packages_path = node_modules_path.join("@biomejs");
  let mut package_names = std::fs::read_dir(biome_packages_path)
    .ok()?
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.file_name().into_string().ok())
    .filter(|name| name.starts_with(BIOME_CLI_PACKAGE_PREFIX))
    .collect::<Vec<_>>();

  package_names.sort();

  for package_name in package_names {
    let binary_name = if package_name.starts_with("cli-win32-") {
      "biome.exe"
    } else {
      "biome"
    };
    let path = node_modules_path
      .join("@biomejs")
      .join(package_name)
      .join(binary_name);

    if std::fs::metadata(&path).is_ok_and(|stat| stat.is_file()) {
      return Some(path);
    }
  }

  None
}

impl BiomeExtension {
  fn extension_server_path(&self) -> Option<PathBuf> {
    extension_server_path_from_node_modules(Path::new(EXTENSION_NODE_MODULES_PATH))
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

    let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

    if self.extension_server_path().is_none()
      || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
    {
      zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
      );
      let result = zed::npm_install_package(PACKAGE_NAME, &version);
      match result {
        Ok(()) => {
          if self.extension_server_path().is_none() {
            Err(format!(
              "installed package '{PACKAGE_NAME}' did not contain an executable @biomejs/cli-* package",
            ))?;
          }
        }
        Err(error) => {
          if self.extension_server_path().is_none() {
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

  fn inline_config(&self, settings: &Value) -> Option<Map<String, Value>> {
    settings
      .get("inline_config")
      .and_then(|value| value.as_object().cloned())
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
        command: binary.path.unwrap_or(WORKTREE_SERVER_PATH.to_string()),
        args: binary.arguments.unwrap_or(args),
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

    if let Some(path) = worktree.which("biome") {
      return Ok(zed::Command {
        command: path,
        args,
        env: Default::default(),
      });
    }

    // install/update and run biome for extension
    self.check_biome_updates(language_server_id)?;

    let server_path = self.extension_server_path().ok_or_else(|| {
      format!("package '{PACKAGE_NAME}' did not contain an executable @biomejs/cli-* package")
    })?;

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
        "inlineConfig": self.inline_config(&settings),
      },
    })))
  }
}

zed::register_extension!(BiomeExtension);

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn temp_node_modules() -> PathBuf {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let path = std::env::temp_dir().join(format!("zed-biome-test-{unique}"));
    fs::create_dir_all(path.join("@biomejs")).unwrap();
    path
  }

  #[test]
  fn finds_linux_cli_installed_by_npm() {
    let node_modules = temp_node_modules();
    let package_dir = node_modules.join("@biomejs").join("cli-linux-x64");
    fs::create_dir_all(&package_dir).unwrap();
    fs::write(package_dir.join("biome"), "").unwrap();

    let path = extension_server_path_from_node_modules(&node_modules).unwrap();

    assert_eq!(path, package_dir.join("biome"));

    fs::remove_dir_all(node_modules).unwrap();
  }

  #[test]
  fn finds_windows_cli_installed_by_npm() {
    let node_modules = temp_node_modules();
    let package_dir = node_modules.join("@biomejs").join("cli-win32-arm64");
    fs::create_dir_all(&package_dir).unwrap();
    fs::write(package_dir.join("biome.exe"), "").unwrap();

    let path = extension_server_path_from_node_modules(&node_modules).unwrap();

    assert_eq!(path, package_dir.join("biome.exe"));

    fs::remove_dir_all(node_modules).unwrap();
  }
}
