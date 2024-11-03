# Sapin

:evergreen_tree: A simple crosshair overlay for PC games

## install

Sapin is a portable executable, no installation is required. \
Download the latest binary from [releases](https://github.com/doums/sapin/releases/latest)
and run it.

## configuration

You can change things like the crosshair color, shape (crosshair or dot),
transparency, size, thickness, gap etc…

Sapin ship with a tray icon, right click on it to access the settings. \
Click _Config_ to open the configuration file location and open
`config.toml` with notepad (or any text editor) to modify it. \
The crosshair overlay is hot reloaded when the config file is saved.

## FAQ

### Oses supported?

Windows (tested on Windows 10 and 11).

### Is it a cheat?

No, Sapin is just an overlay, it doesn't interact with the game in any way. \
It opens a window on top of all other windows, centered, and draws the crosshair on it. \
Feel free to check the source code.

### Which games are supported?

It should work with most games (in fullscreen and fullscreen windowed modes).

### It doesn't work with my game

Some games block overlays, you can try to run the game in windowed mode.

### Can I get banned for using Sapin?

-> [Is it a cheat?](#is-it-a-cheat) \
I can't guarantee that you won't get banned, use at your own risk.

### Windows flags it as a virus

It's a false positive, Sapin is open source, feel free to check the source code. \
Signing the executable would fix the issue, but it costs money, so I'm not
planning to do it. \
If Windows Defender is blocking the executable, you can add an exception for it.

### The crosshair is not pixel perfect or a bit blurry

This could be due to Windows display scaling factor. \
Check in settings: _System_ > _Display_ > _Scale_

## development

#### prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [bun](https://bun.sh/)
- [tauri](https://v2.tauri.app/start/prerequisites/)

Install the dependencies

```shell
bun i
```

#### dev

```shell
bun tauri dev
```

Custom log level can be set via the `RUST_LOG` env variable, for example:
`RUST_LOG=info,sapin=trace`

#### build

```shell
bun tauri build
```

## license

MPL-2.0

---

_built using Rust, [tauri](https://v2.tauri.app/), [Svelte](https://svelte.dev/) and HTML canvas_
