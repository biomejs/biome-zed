# Biome - Zed

<div align="center">

[![CI main](https://github.com/biomejs/biome-zed/actions/workflows/main.yml/badge.svg)](https://github.com/biomejs/biome-zed/actions/workflows/main.yml)

</div>

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

## Installation

Requires Zed >= **v0.131.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Biome_.

## Configuration

Example configurations in zed `settings.json`.

```json5
// settings.json
{
  "format_on_save": "on",
  "code_actions_on_format": {
    "source.fixAll": true,
    "source.organizeImports.biome": true
  },
  "formatter": "language_server"
}
```
