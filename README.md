# Rust learnings

## Installation Notes

- don't open VSCode and just open a directory, gotta do `cargo new [project name]` so it makes the lock and toml config files so the analyzer can see it
- add this to the vscode config:
```json
"[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
}
```
