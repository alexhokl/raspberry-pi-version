# raspberry-pi-version

This shows the version information of a Raspberry Pi.

## Prerequisite

Docker with access to Intel CPU (or its emulation).

```sh
cargo install cross
```

## Build

```sh
task build
```

## Development tricks

To add a build target for Raspberry Pi 4

```sh
rustup target add armv7-unknown-linux-gnueabihf
```
