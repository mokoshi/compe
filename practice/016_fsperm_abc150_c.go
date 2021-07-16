// https://atcoder.jp/contests/abc150/tasks/abc150_c
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"reflect"
	"strconv"
)

func main() {
	n := readInt()
	p := make([]int, n)
	for i := 0; i < n; i++ {
		p[i] = readInt()
	}
	q := make([]int, n)
	for i := 0; i < n; i++ {
		q[i] = readInt()
	}
	perm := make([]int, n)
	for i := 0; i < n; i++ {
		perm[i] = i + 1
	}

	var permutations func([]int) [][]int
	permutations = func(pp []int) [][]int {
		if len(pp) == 0 {
			return [][]int{}
		} else if len(pp) == 1 {
			return [][]int{pp}
		}

		var res [][]int
		for i := 0; i < len(pp); i++ {
			next := append([]int{}, pp[:i]...)
			next = append(next, pp[i+1:]...)
			patterns := permutations(next)
			for _, pattern := range patterns {
				add := append([]int{pp[i]}, pattern...)
				res = append(res, add)
			}
		}
		return res
	}

	a := 0
	b := 0
	for i, permutation := range permutations(perm) {
		if reflect.DeepEqual(p, permutation) {
			a = i
		}
		if reflect.DeepEqual(q, permutation) {
			b = i
		}
	}
	fmt.Println(math.Abs(float64(a) - float64(b)))
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
