package main

import "fmt"

var A1 int

func init() {
	A1 = 10
	fmt.Println("This is a.go, Hello")
}

func A() {
	fmt.Println("This is a.go, A()")
}
