<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
        <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
    </picture>
</p>

<div align="center">

[![CI main](https://github.com/biomejs/biome-zed/actions/workflows/main.yml/badge.svg)](https://github.com/biomejs/biome-zed/actions/workflows/main.yml)

</div>

# Biome extension for Zed

This extension adds support for [Biome](https://github.com/biomejs/biome) in [Zed](https://zed.dev/).

Languages currently supported:

- **JavaScript**
- **TypeScript**
- **JSX**
- **TSX**
- **JSON**
- **JSONC**
- **Vue.js**
- **Astro**
- **Svelte**
- **CSS**

## Installation

Requires Zed >= **v0.131.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Biome_.

## Configuration

By default, the biome.json file is required to be in the **root of the workspace**.

Otherwise, it can be configured through the lsp settings:

```jsonc
// settings.json
{
  "lsp": {
    "biome": {
      "settings": {
        "config_path": "<path>/biome.json"
      }
    }
  }
}
```

### Formatting

**Formatting does not work through the extension yet.**

Instead, you can configure biome as an external formatter:

```jsonc
// settings.json
{
  "formatter": {
    "external": {
      "command": "./node_modules/@biomejs/biome/bin/biome",
      "arguments": ["format", "--write", "--stdin-file-path", "{buffer_path}"]
    }
  }
}
```

### Enable biome only when biome.json is present

```jsonc
// settings.json
{
  "lsp": {
    "biome": {
      "settings": {
        "require_config_file": true
      }
    }
  }
}
```

### Project based configuration

If you'd like to exclude biome from running in every project,

1. Disable the biome language server in user settings:

```jsonc
// settings.json
{
  "language_servers": [ "!biome", "..." ]
}
```

2. And enable it in the projects local settings:

```jsonc
// <workspace>/.zed/settings.json
{
  "language_servers": [ "biome", "..." ]
}
```

The same can be configured on a per-lanugage bassis with the [`languages`](https://zed.dev/docs/configuring-zed#languages) key.

### Run code actions on format:

```jsonc
// settings.json
{
  "code_actions_on_format": {
    "source.fixAll.biome": true,
    "source.organizeImports.biome": true
  }
}
```
