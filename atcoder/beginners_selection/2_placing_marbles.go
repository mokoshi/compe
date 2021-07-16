package main

import (
	"fmt"
)

func main() {
	var a string
	fmt.Scanf("%s", &a)

	var result = 0
	if a[0:1] == "1" {
		result++
	}
	if a[1:2] == "1" {
		result++
	}
	if a[2:3] == "1" {
		result++
	}
	fmt.Print(result)
}
