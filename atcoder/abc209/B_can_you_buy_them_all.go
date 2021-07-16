package main

import (
	"fmt"
)

func main() {
	var n, x int
	fmt.Scan(&n, &x)

	total := 0
	for i := 1; i <= n; i++ {
		var a int
		fmt.Scan(&a)

		total += a - (1 - i%2)
	}

	if total <= x {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
