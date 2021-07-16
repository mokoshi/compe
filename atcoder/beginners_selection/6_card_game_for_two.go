package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	fmt.Scanf("%d", &n)

	var nums []int
	for i := 0; i < n; i++ {
		var num int
		fmt.Scan(&num)
		nums = append(nums, num)
	}

	sort.Slice(nums, func(i, j int) bool {
		return nums[i] > nums[j]
	})

	result := 0
	for i, num := range nums {
		result += num * (1 - (i % 2 * 2))
	}

	fmt.Print(result)
}
