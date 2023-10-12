package main

/*
#cgo CFLAGS: -I./include
#cgo LDFLAGS: -L./lib -lvpnym
#include "vpnym.h"
*/
import "C"

import (
	"fmt"
)

func main() {
	fmt.Println("calling libvpnym…")
	C.run_tauri()
}
