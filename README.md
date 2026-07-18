# Ashenpaper

A lightweight GUI wallpaper switcher for [hyprpaper](https://github.com/hyprwm/hyprpaper), built with Rust and `egui`.

> **Status: in development, not yet usable.** The window currently lists wallpapers from a folder but does not yet support clicking to apply one. Core logic for setting wallpapers via `hyprctl` works, but isn't wired into the UI yet.

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
