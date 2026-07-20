# Ashenpaper

A lightweight GUI wallpaper switcher for [hyprpaper](https://github.com/hyprwm/hyprpaper), built with Rust and `egui`.

> **Status: in development, missing core features** Applying a wallpaper won't change it permanently, only to the current running hyprpaper daemon, ui design is on a very early stage.

## Goal

Browse the wallpapers in a folder and apply one across all connected monitors with a click.

## Requirements

- [Hyprland](https://hyprland.org/) + [hyprpaper](https://github.com/hyprwm/hyprpaper) running, with `ipc = on` set in `hyprpaper.conf`
- Rust toolchain (to build from source)

## Build

```bash
cargo build --release
```

## How it works (so far)

Ashenpaper reads monitor names via `hyprctl monitors -j` and can apply a wallpaper to every connected monitor using `hyprctl hyprpaper wallpaper`. Wallpapers are read from `~/Pictures/Wallpapers`.
