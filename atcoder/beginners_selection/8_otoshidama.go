package main

import (
	"fmt"
)

func main() {
	var n, y int
	fmt.Scan(&n, &y)

	fmt.Print(check(n, y))
}

func check(n, y int) (int, int, int) {
	for i := 0; i <= n; i++ {
		for j := 0; j <= n-i; j++ {
			k := n - i - j
			if 10000*i+5000*j+1000*k == y {
				return i, j, k
			}
		}
	}
	return -1, -1, -1
}
