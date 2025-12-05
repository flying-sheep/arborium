# arborium-theme

Theme and highlight definitions for [arborium](https://github.com/bearcove/arborium) syntax highlighting.

This crate provides:

- **Highlight definitions**: Mapping from tree-sitter capture names to short HTML tags (e.g., `keyword` -> `<a-k>`)
- **Theme types**: `Theme`, `Color`, `Style` for representing syntax highlighting themes
- **Built-in themes**: 24 popular color schemes ready to use

## Usage

```rust
use arborium_theme::{Theme, builtin, HIGHLIGHTS};

// Use a built-in theme
let theme = builtin::catppuccin_mocha();

// Generate CSS for the theme
let css = theme.to_css("[data-theme=\"mocha\"]");

// Access highlight definitions
for def in HIGHLIGHTS {
    println!("{} -> <a-{}>", def.name, def.tag);
}
```

## Built-in Themes

This crate includes 24 themes from popular color schemes. We are grateful to the original theme authors:

| Theme | Source |
|-------|--------|
| Ayu Dark | [ayu-theme/ayu-colors](https://github.com/ayu-theme/ayu-colors) |
| Ayu Light | [ayu-theme/ayu-colors](https://github.com/ayu-theme/ayu-colors) |
| Catppuccin Frappe | [catppuccin/catppuccin](https://github.com/catppuccin/catppuccin) |
| Catppuccin Latte | [catppuccin/catppuccin](https://github.com/catppuccin/catppuccin) |
| Catppuccin Macchiato | [catppuccin/catppuccin](https://github.com/catppuccin/catppuccin) |
| Catppuccin Mocha | [catppuccin/catppuccin](https://github.com/catppuccin/catppuccin) |
| Dracula | [draculatheme.com](https://draculatheme.com) |
| ef-melissa-dark | [protesilaos.com/emacs/ef-themes](https://protesilaos.com/emacs/ef-themes) |
| GitHub Dark | [primer/github-vscode-theme](https://github.com/primer/github-vscode-theme) |
| GitHub Light | [primer/github-vscode-theme](https://github.com/primer/github-vscode-theme) |
| Gruvbox Dark | [morhetz/gruvbox](https://github.com/morhetz/gruvbox) |
| Gruvbox Light | [morhetz/gruvbox](https://github.com/morhetz/gruvbox) |
| Kanagawa Dragon | [rebelot/kanagawa.nvim](https://github.com/rebelot/kanagawa.nvim) |
| Light Owl | [sdras/night-owl-vscode-theme](https://github.com/sdras/night-owl-vscode-theme) |
| Lucius Light | [jonathanfilip/vim-lucius](https://github.com/jonathanfilip/vim-lucius) |
| Melange Dark | [savq/melange-nvim](https://github.com/savq/melange-nvim) |
| Melange Light | [savq/melange-nvim](https://github.com/savq/melange-nvim) |
| Monokai Pro | [monokai.pro](https://monokai.pro) |
| Nord | [nordtheme.com](https://www.nordtheme.com) |
| One Dark | [atom/one-dark-syntax](https://github.com/atom/one-dark-syntax) |
| Rose Pine Moon | [rosepinetheme.com](https://rosepinetheme.com) |
| Solarized Dark | [ethanschoonover.com/solarized](https://ethanschoonover.com/solarized/) |
| Solarized Light | [ethanschoonover.com/solarized](https://ethanschoonover.com/solarized/) |
| Tokyo Night | [enkia/tokyo-night-vscode-theme](https://github.com/enkia/tokyo-night-vscode-theme) |

## License

This crate is licensed under MIT OR Apache-2.0.

The built-in themes are adaptations of color schemes from their respective projects. Please see each project's repository for their specific licensing terms.
