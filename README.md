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
`config.toml` with a text editor to modify it. \
The crosshair overlay is hot reloaded when the config file is saved.

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

```shell

#### build

```shell
bun tauri build
```

## license

MPL-2.0

_built using Rust, [tauri](https://v2.tauri.app/), [Svelte](https://svelte.dev/) and HTML canvas_
