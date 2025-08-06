# Quickstitch Applications

This repository features a sample CLI and GUI applicaiton built around [Quickstitch](https://github.com/quietkiro/quickstitch).

If you'd like to give either application a spin, there's a sample chapter drawn by [Leafsky](https://www.instagram.com/_.melo.vee._/) located [here](https://github.com/quietkiro/quickstitch_bin/blob/main/test-sample.zip), distributed with her permission.

## Getting Started with the GUI

If you have a Linux or Windows system, do check out the [releases page](https://github.com/quietkiro/quickstitch_bin/releases) and download the executable that suits your system.

If you would like to compile it for yourself, ensure your system has Rust installed along with Cargo. After doing so, clone this repository, and run the GUI application with the following command:
```
cargo run --bin quickstitch_gui
```

## Getting Started with the CLI

Installing the CLI can be done on a system with Rust and Cargo installed by performing:
```
cargo install --git https://github.com/quietkiro/quickstitch_bin
```

After which, performing the following command should be enough for you to get started.
```
quickstitch_cli --help
```
