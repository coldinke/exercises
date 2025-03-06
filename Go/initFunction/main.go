package main

import (
	"fmt"
)

func init() {
	fmt.Println("This is first init function")
}

func init() {
	fmt.Println("This is second init function")
}

func main() {
	fmt.Println("vim-go from main")

	B()
	A()
}

func init() {
	fmt.Println("This is three init function")
}
