package main

import "fmt"

func init() {
	fmt.Println("This is b.go, Hello")
	fmt.Println("This is b.go, this value from the a.go, value: ", A1)
}

func B() {
	fmt.Println("This is b.go, B()")
}
