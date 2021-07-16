package main

import (
	"fmt"
)

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	if a < b {
		fmt.Println(b - a + 1)
	} else {
		fmt.Println(0)
	}
}
