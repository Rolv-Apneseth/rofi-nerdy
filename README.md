<h1 align="center">rofi-nerdy</h1>

<p align="center">Nerd font icon selector plugin for rofi</p>

<p align="center">
  <img src="https://img.shields.io/badge/License-AGPL_v3-green.svg" alt="License: AGPL v3" />
  <img src="https://img.shields.io/github/v/tag/rolv-apneseth/rofi-nerdy?label=version&color=blueviolet" alt="version" />
  <a href="https://aur.archlinux.org/packages/rofi-nerdy"><img src="https://img.shields.io/aur/version/rofi-nerdy" alt="AUR version" /></a>
</p>

![rofi-nerdy - demo image](https://github.com/Rolv-Apneseth/rofi-nerdy/assets/69486699/9dfb6cd0-17aa-4571-859e-94523327710a)

## Dependencies

Commands used vary depending on your chosen display server:

### Wayland

- [wl-clipboard](https://github.com/bugaevc/wl-clipboard)
- [wtype](https://github.com/atx/wtype)

---

### X11

- [xclip](https://github.com/astrand/xclip)
- [xdotool](https://github.com/jordansissel/xdotool)

## Installation

### AUR

```bash
paru -S rofi-nerdy
```

---

### just

1. Clone repo:

    ```bash
    git clone https://github.com/Rolv-Apneseth/rofi-nerdy.git
    ```

2. Use `just` to install (requires `cargo` and `just`)

    ```bash
    cd rofi-nerdy && sudo just install
    ```

---

### Manual (not recommended)

```bash
git clone https://github.com/Rolv-Apneseth/rofi-nerdy.git
cd rofi-nerdy
cargo build --release --lib
sudo cp target/release/librofi_nerdy.so /usr/lib/rofi/nerdy.so
```

If you are using the latest changes from the rofi repo (e.g. rofi-lbonn-wayland-git, rofi-git), then the build step needs to be preceded by RUSTFLAGS="--cfg rofi_next" for it to work

## Usage

After installing, simply run the following command:

```bash
rofi -modi nerdy -show nerdy
```

However, I also recommend setting a theme with the `-theme` flag. To achieve what is shown in the demo image, have a look at [my rofi config](https://github.com/Rolv-Apneseth/.dotfiles/tree/main/rofi/.config/rofi), specifically the `icons.rasi` file (follow the import chain for the full theme).

## Keybinds

| Keybind           | Default rofi keybind              | Action                 |
|-------------------|-----------------------------------|------------------------|
| `kb-accept-entry` | <kbd>Enter</kbd>                  | Copy icon              |
| `kb-accept-alt`   | <kbd>Shift</kbd>+<kbd>Enter</kbd> | Attempt to insert icon |

- To change a `rofi` keybind, you can, for example, use `-kb-accept-entry Ctrl+c`

## Acknowledgement

The creation of this plugin was directly inspired by [nerdy.nvim](https://github.com/2KAbhishek/nerdy.nvim), so thank you to [@2KAbhishek](https://github.com/2KAbhishek).

Of course, thank you also to the creator and maintainers of the [nerd fonts](https://github.com/ryanoasis/nerd-fonts) themselves.
