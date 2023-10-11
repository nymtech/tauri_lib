package main

/*
#cgo LDFLAGS: -L./lib -lvpnym
#include "./lib/vpnym.h"
*/
import "C"

import (
	"fmt"
)

func main() {
	fmt.Println("calling libvpnym…")
	C.run_tauri()
}
