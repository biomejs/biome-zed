# Biome - Zed

This extension adds support for [Biome](https://github.com/biomejs/biome) in [Zed](https://zed.dev/).

## Configure Formatting

Example formatter configurations in zed `settings.json`.

```jsonc
// settings.json
{
  // Format on save
  "format_on_save": "on",

  "formatter": {
    "code_actions": {
      // Run fixAll on format.
      "source.fixAll": true,
      // Organize Imports
      "source.organizeImports": true
    }
  }
}
```
