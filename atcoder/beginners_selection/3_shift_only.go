package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scanf("%d", &n)

	var nums []int
	for i := 0; i < n; i++ {
		var num int
		fmt.Scanf("%d", &num)
		nums = append(nums, num)
	}

	fmt.Print(countLoop(nums))
}

func countLoop(nums []int) int {
	loop := 0
	for true {
		for i, num := range nums {
			if num%2 == 0 {
				nums[i] = num / 2
			} else {
				return loop
			}
		}
		loop++
	}
	return loop
}
