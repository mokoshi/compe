package main

import (
	"fmt"
)

func main() {
	var a, b, c, x int
	fmt.Scan(&a)
	fmt.Scan(&b)
	fmt.Scan(&c)
	fmt.Scan(&x)

	patterns := 0
	for i := 0; i <= a; i++ {
		r1 := x - i * 500
		if r1 <= 0 {
			if r1 == 0 {
				patterns++
			}
			break
		}
		for j := 0; j <= b; j++ {
			r2 := r1 - j * 100
			if r2 <= 0 {
				if r2 == 0 {
					patterns++
				}
				break
			}
			if r2 % 50 == 0 && r2 / 50 <= c {
				patterns ++
			}
		}
	}

	fmt.Print(patterns)
}
