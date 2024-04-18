alias b := build
alias i := install
alias u := uninstall
alias c := clean
alias t := test
alias tb := test-bare
alias d := develop

# VARIABLES ----------------------------------------------------------------------------------------
PKGNAME := env("PKGNAME", "rofi-nerdy")
PKGDIR := env("PKGDIR", "")

LIB_NAME := "librofi_nerdy.so"
PLUGIN_NAME := "nerdy.so"

LICENSES_DIR := "/usr/share/licenses/" + PKGNAME

PLUGINS_DIR := `pkg-config --variable pluginsdir rofi || echo "/lib/rofi"`
PLUGIN_PATH := join(PLUGINS_DIR, PLUGIN_NAME)

# Set rust flags if running a version of `rofi` with changes newer than the base `1.7.5`
# See https://github.com/SabrinaJewson/rofi-mode.rs/issues/8#event-11112343153
# Examples of version outputs
#     rofi: Version: 1.7.5
#     rofi(wayland): Version: 1.7.5+wayland2
#     rofi-git: Version: 1.7.5-187-gb43a82f8 (makepkg)
#     rofi-lbonn-wayland-git: Version: 1.7.5+wayland2-154-g36621af0 (makepkg)
RUSTFLAGS := if `rofi -version` =~ '^Version: 1\.7\.5(?:\+wayland2)?$' { "" } else { "--cfg rofi_next" }

# COMMANDS -----------------------------------------------------------------------------------------
# List commands
default:
    @just --list

# Update Nerd font icons
update-nerd:
    #!/bin/env python3
    # Fetches and converts the `glyphnames.json` file from the below URL to a rust source file for easy usage

    import requests

    data = requests.get("https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/glyphnames.json").json()

    with open("src/icons.rs", "w") as output_file:
        output_file.write("// Automatically generated - don't edit this file directly\n")
        output_file.write("pub fn get_icons() -> Vec<[&'static str; 2]> {\n    vec![\n")

        for key in data:
            if key == "METADATA":
                continue

            char = str(data[key]["char"])
            code = str(data[key]["code"])

            output_file.write(f"""        ["{char}", "{key}"],\n""")

        output_file.write("    ]\n}\n")

# Build
build:
    RUSTFLAGS="{{RUSTFLAGS}}" cargo build --release --lib

# Build + install
install: build
    # Plugin
    install -DT "target/release/{{LIB_NAME}}" "{{clean(PKGDIR + "/" + PLUGIN_PATH)}}"

    # License
    install -Dt "{{PKGDIR}}{{LICENSES_DIR}}" LICENSE

    cargo clean

# Uninstall
uninstall:
    rm {{PLUGIN_PATH}}
    rm -rf {{LICENSES_DIR}}

# Clean
clean:
    cargo clean --verbose

# Run with specific theme
test $THEME:
    rofi -modi nerdy -show nerdy -theme $THEME

# Run with no theme
test-bare:
    rofi -modi nerdy -show nerdy

# Rebuild and replace plugin file whenever a `.rs` file is updated
develop:
    fd --extension rs | entr -s 'RUSTFLAGS="{{RUSTFLAGS}}" cargo build --lib && sudo cp --force target/debug/{{LIB_NAME}} {{PLUGIN_PATH}}'
