# Biome - Zed

This extension adds support for [Biome](https://github.com/biomejs/biome) in [Zed](https://zed.dev/).

Currently supports **JavaScript**, **TypeScript**, **TSX**, **Vue.js**, **Astro** and **Svelte** files.

## Installtion

Requires Zed >= **v0.131.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Biome_.

## Configuration

Example configurations in zed `settings.json`.

```jsonc
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
