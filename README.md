# Ashenpaper

A lightweight GUI wallpaper switcher for [hyprpaper](https://github.com/hyprwm/hyprpaper), built with Rust and `egui`.

Browse the wallpapers in a folder in a centered, responsive grid, each thumbnail cropped to your monitor's aspect ratio, and apply one across all connected monitors with a click. The choice persists across restarts.

## Requirements

- [Hyprland](https://hyprland.org/) + [hyprpaper](https://github.com/hyprwm/hyprpaper) running, with `ipc = on` set in `hyprpaper.conf`
- Rust toolchain (to build from source)

## Build

```bash
cargo build --release
```

## Usage

```bash
./target/release/ashenpaper
```

Wallpapers are read from `~/Pictures/Wallpapers` by default.

## How it works

Ashenpaper reads monitor names and resolution via `hyprctl monitors -j`, using the primary monitor's aspect ratio to crop thumbnails so they preview accurately. Clicking a wallpaper applies it to every connected monitor via `hyprctl hyprpaper wallpaper`, and rewrites `hyprpaper.conf` on disk so the choice survives a restart.