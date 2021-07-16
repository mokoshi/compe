package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	fmt.Scanf("%d", &n)

	nums := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&nums[i])
	}

	sort.Ints(nums)

	count := 0
	prev := 0
	for _, num := range nums {
		if prev < num {
			count++
		}
		prev = num
	}

	fmt.Print(count)
}
