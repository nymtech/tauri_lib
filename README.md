## Tauri lib POC

This is a demonstration of a tauri app compiled as a dynamic
shared library, that can be run from another app

## Install

#### Prerequisites

- Rust
- Nodejs (npm), latest LTS version recommended

Some system libraries are required depending on the host platform.
Follow the instructions for your specific OS [here](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Install project dependencies

```
npm i
```

## App Development

```
cd app
npm run app:dev
```

## Build

build the app (lib)

```
cd app
npm run app:build
```

build the runner

```
cd runner
cargo build
```

## Run

First copy the compiled shared library in a subdirectory named
`lib` relative to where you want to run the app:

Example on Linux:

```
cd runner
mkdir lib
cp ../app/src-tauri/target/release/libvpnym.so lib
cargo run
```

