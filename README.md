# bamhls

An HLS format playlist file sorting utility

## About

For information regarding implementation, see [PROCESS.md](./PROCESS.md). For
more information on using this application, read on.

## Usage

```
$ bamhls <master playlist uri>
```

## Installation

This application can be installed as a precompiled binary or from source.

### Precompiled builds

See the [releases page](https://github.com/g-s-k/bamhls/releases/latest) for
pre-built binaries. They are located in the "Assets" section of each release.

### From source

If you wish to install the bleeding edge version, or if your platform does not
have a pre-built binary on the releases page, you can do the following:

1. Clone this repository to your computer.
2. Ensure you have the Rust toolchain installed (see
   [rustup](https://rustup.sh/) for a relatively quick and well-supported way
   to do this).
3. Enter this directory on the command line and run `cargo install`.
