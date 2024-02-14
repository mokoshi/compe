// https://atcoder.jp/contests/abc145/tasks/abc145_c
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	x := make([]int, n)
	y := make([]int, n)
	p := make([]int, n)
	for i := 0; i < n; i++ {
		p[i] = i
		x[i] = readInt()
		y[i] = readInt()
	}

	var total float64 = 0
	cnt := 0
	for ok := true; ok; ok = nextPermutation(p) {
		for i := 0; i < n-1; i++ {
			s := p[i]
			t := p[i+1]
			total += math.Sqrt(float64((x[s]-x[t])*(x[s]-x[t]) + (y[s]-y[t])*(y[s]-y[t])))
		}

		cnt++
	}
	fmt.Println(total / float64(cnt))
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
