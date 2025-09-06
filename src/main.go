package main

import (
	"fmt"
	"os"
)

func main() {
	bytes, _ := os.ReadFile("./examples/00.lang")
	source := string(bytes)
	fmt.Print(source)
}
