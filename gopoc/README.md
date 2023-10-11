## gopoc

POC of a Go program consuming a tauri app as a shared library
through FFI

For the FFI layer, between Go and Rust, we are using [cgo](https://pkg.go.dev/cmd/cgo)

### copying tauri lib files

First copy the lib files (depending on what's your host/target arch) into a `lib` directory

```
.
├── go.mod
├── gopoc.go
├── lib
│  ├── libvpnym.so
│  ├── vpnym.dll
│  ├── vpnym.h
│  └── WebView2Loader.dll
└── README.md
```

### dev

```
WEBKIT_DISABLE_COMPOSITING_MODE=1 go run -ldflags="-r ./lib" gopoc.go
```

### build

```
go build -ldflags="-r ./lib" -o bin/vpnym_d gopoc.go
```

To build for Windows x86_64 architecture (from Linux)

#### prerequisite

MinGW-w64 cross-compiler

```
GOOS=windows GOARCH=amd64 CGO_ENABLED=1 CXX=x86_64-w64-mingw32-g++ CC=x86_64-w64-mingw32-gcc go build -ldflags="-r ./lib" -o bin/vpnym_d.exe gopoc.go
```

### Windows packaging

Make a bundle of these files
- `vpnym_d.exe` (go binary executable)
- `vpnym.dll` (tauri app as a lib)
- `WebView2Loader.dll` (library needed by tauri)

