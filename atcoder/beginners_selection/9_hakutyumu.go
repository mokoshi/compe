package main

import (
	"fmt"
	"regexp"
)

func main() {
	var s string
	fmt.Scan(&s)

	r := regexp.MustCompile("^(dream|dreamer|erase|eraser)+$")
	if r.MatchString(s) {
		fmt.Print("YES")
	} else {
		fmt.Print("NO")
	}
}
