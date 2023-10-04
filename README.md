## Tauri lib POC

This is a demonstration of a tauri app compiled as a dynamic
shared library, that can be run from another app

## Install

#### Prerequisites

Follow instructions for your specific platform → https://tauri.app/v1/guides/getting-started/prerequisites

#### Install project dependencies

```
npm i
```

## Build

build the app (lib)

```
cd app/src-tauri
npm run build
cargo build
```

build the runner

```
cd runner
cargo build
cargo run
```