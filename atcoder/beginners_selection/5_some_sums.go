package main

import (
	"fmt"
	"strconv"
)

func main() {
	var n, a, b int
	fmt.Scan(&n, &a, &b)

	result := 0
	for i := 1; i <= n; i++ {
		s := strconv.Itoa(i)

		total := 0
		for _, k := range s {
			total += int(k - 48)
		}

		if total >= a && total <= b {
			result += i
		}
	}

	fmt.Print(result)
}
