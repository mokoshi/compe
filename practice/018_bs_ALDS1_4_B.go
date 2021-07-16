// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B&lang=ja
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	s := make([]int, n)
	for i := 0; i < n; i++ {
		s[i] = readInt()
	}
	q := readInt()
	t := make([]int, q)
	for i := 0; i < q; i++ {
		t[i] = readInt()
	}

	var isIncluded func([]int, int) bool
	isIncluded = func(nums []int, num int) bool {
		if len(nums) == 1 {
			return nums[0] == num
		}
		c := len(nums)/2 - 1
		leftMax := nums[c]
		if num <= leftMax {
			return isIncluded(nums[:c+1], num)
		} else {
			return isIncluded(nums[c+1:], num)
		}
	}

	ans := 0
	for i := 0; i < q; i++ {
		if isIncluded(s, t[i]) {
			ans++
		}
	}
	fmt.Println(ans)
}

const MaxBuf = 200100

var buf []byte = make([]byte, MaxBuf)
var sc = bufio.NewScanner(os.Stdin)

func init() {
	sc.Split(bufio.ScanWords)
	sc.Buffer(buf, MaxBuf)
}

func readString() string {
	sc.Scan()
	return sc.Text()
}

func readInt() int {
	sc.Scan()
	r, _ := strconv.Atoi(sc.Text())
	return r
}

func readInt64() int64 {
	sc.Scan()
	r, _ := strconv.ParseInt(sc.Text(), 10, 64)
	return r
}

func readFloat64() float64 {
	sc.Scan()
	r, _ := strconv.ParseFloat(sc.Text(), 64)
	return r
}

func nextPermutation(a []int) bool {
	return permutationPandita(a, func(x, y int) bool { return x < y })
}

func prevPermutation(a []int) bool {
	return permutationPandita(a, func(x, y int) bool { return y < x })
}

func permutationPandita(a []int, less func(x, y int) bool) bool {
	i := len(a) - 2
	for 0 <= i && !less(a[i], a[i+1]) {
		i--
	}
	if i == -1 {
		return false
	}
	j := i + 1
	for k := j + 1; k < len(a); k++ {
		if less(a[i], a[k]) && !less(a[j], a[k]) {
			j = k
		}
	}
	a[i], a[j] = a[j], a[i]
	for p, q := i+1, len(a)-1; p < q; p, q = p+1, q-1 {
		a[p], a[q] = a[q], a[p]
	}
	return true
}
