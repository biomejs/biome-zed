# Biome - Zed

This extension adds support for [Biome](https://github.com/biomejs/biome) in Zed.

## Formatting

Formatter configurations in zed `settings.json`.

### Format on save

```json
// settings.json
{
  "format_on_save": "on",
}
```

### Code Actions

Run fixAll on format.

```json
// settings.json
{
  "formatter": {
    "code_actions": {
      "source.fixAll": true,
    }
  }
}
```

### Organize Imports

```json
// settings.json
{
  "formatter": {
    "code_actions": {
      "source.organizeImports": true
    }
  }
}
```
