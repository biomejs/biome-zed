# Biome - Zed

This extension adds support for [Biome](https://github.com/biomejs/biome) in [Zed](https://zed.dev/).

## Configuration

Example configurations in zed `settings.json`.

```jsonc
// settings.json
{
  "format_on_save": "on",
  "code_actions_on_format": {
    "source.fixAll": true,
    "source.organizeImports": true
  },
  "formatter": {
    "external": {
      "command": "./node_modules/@biomejs/biome/bin/biome",
      "arguments": [
        "format",
        "--write",
        "--stdin-file-path",
        "{buffer_path}"
      ]
    }
  }
}
```
