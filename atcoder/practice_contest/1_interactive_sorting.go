package main

import (
	"fmt"
	"strings"
)

func main() {
	var n, q int
	fmt.Scan(&n, &q)

	var balls = make([]string, n)
	for i := 0; i < n; i++ {
		balls[i] = string(rune('A' + i))
	}

	ans := mergeSortInteractive(balls)
	//ans := quickSortInteractive(balls)

	fmt.Printf("! %s", strings.Join(ans, ""))
}

func quickSortInteractive(balls []string) []string {
	if len(balls) <= 1 {
		return balls
	}

	var smaller, larger []string
	pivot := balls[len(balls)-1]
	for _, ball := range balls {
		if pivot == ball {
			continue
		}
		fmt.Printf("? %s %s\n", ball, pivot)

		var ans string
		fmt.Scan(&ans)

		if ans == "<" {
			smaller = append(smaller, ball)
		} else {
			larger = append(larger, ball)
		}
	}

	sorted := quickSortInteractive(smaller)
	sorted = append(sorted, pivot)
	sorted = append(sorted, quickSortInteractive(larger)...)
	return sorted
}

func mergeSortInteractive(balls []string) []string {
	if len(balls) <= 1 {
		return balls
	}

	left := mergeSortInteractive(balls[:len(balls)/2])
	right := mergeSortInteractive(balls[len(balls)/2:])

	var con []string
	for len(left) > 0 && len(right) > 0 {
		leftTop := left[0]
		rightTop := right[0]

		fmt.Printf("? %s %s\n", leftTop, rightTop)

		var ans string
		fmt.Scan(&ans)

		if ans == "<" {
			con = append(con, leftTop)
			left = left[1:]
		} else {
			con = append(con, rightTop)
			right = right[1:]
		}
	}

	con = append(con, left...)
	con = append(con, right...)
	return con
}
