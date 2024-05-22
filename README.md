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
