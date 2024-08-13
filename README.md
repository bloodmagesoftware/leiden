# Leiden

## Play

- [In your browser](https://bloodmagesoftware.github.io/leiden/) Due to permission restrictions, the web version offers
  a limited experience. For the full experience, download the game or build it from source.
- [Download from GitHub Releases](https://github.com/bloodmagesoftware/leiden/releases)

## Build from source

This is a regular [Rust](https://rust-lang.org/) project,
no additional requirements.

Some Features are platform dependent and are disabled by default.
To enable them, use the `--features` flag,
followed by a comma-separated list of features without spaces.

Example: `cargo run --features feature1,feature2,feature3`

**Features:**

| Feature  | Description                                | Windows            | Linux              | macOS              | Web                |
|----------|--------------------------------------------|--------------------|--------------------|--------------------|--------------------|
| `dev`    | Skips some stuff for easier dev experience | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| `exit`   | Add an exit button to the main menu        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| `rumble` | Enable gamepad rumble (vibration)          | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| `sdl`    | Use SDL2 instead of default Bevy rumble    | :x:                | :x:                | :heavy_check_mark: | :x:                |

## Credits

### Font

- [Ornatix Font by designstation](https://www.fontspace.com/ornatix-font-f8043) (Freeware)

### Music

- [The Detective - Christoffer Moe Ditlevsen](https://www.epidemicsound.com/track/MGgv4idBGB/)

### Sound Effects

- [Epidemic Sound](https://www.epidemicsound.com/)
