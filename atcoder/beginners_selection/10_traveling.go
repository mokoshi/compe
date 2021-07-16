package main

import (
	"fmt"
	"math"
)

func main() {
	var n int
	fmt.Scanf("%d", &n)

	prevT, prevX, prevY := 0, 0, 0
	for i := 0; i < n; i++ {
		var t, x, y int
		fmt.Scan(&t, &x, &y)

		dist := int(math.Abs(float64(x-prevX)) + math.Abs(float64(y-prevY)))
		if dist > t-prevT || dist%2 != (t-prevT)%2 {
			fmt.Printf("No")
			return
		}

		prevT = t
		prevX = x
		prevY = y
	}

	fmt.Printf("Yes")
}
