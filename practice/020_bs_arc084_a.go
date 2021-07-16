// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
)

func main() {
	n := readInt()
	a := make([]int, n)
	b := make([]int, n)
	c := make([]int, n)
	for i := 0; i < n; i++ {
		a[i] = readInt()
	}
	for i := 0; i < n; i++ {
		b[i] = readInt()
	}
	for i := 0; i < n; i++ {
		c[i] = readInt()
	}
	sort.Ints(a)
	sort.Ints(b)
	sort.Ints(c)

	ans := 0
	for i := 0; i < n; i++ {
		ap := binarySearch(-1, n, func(idx int) bool {
			return a[idx] < b[i]
		})
		cp := binarySearch(n, -1, func(idx int) bool {
			return c[idx] > b[i]
		})
		ans += (ap + 1) * (n - cp)
	}

	fmt.Println(ans)
}

func binarySearch(ok int, ng int, check func(i int) bool) int {
	for abs(ok-ng) > 1 {
		mid := (ok + ng) / 2
		if check(mid) {
			ok = mid
		} else {
			ng = mid
		}
	}
	return ok
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

func abs(a int) int {
	return int(math.Abs(float64(a)))
}

func max(a int, b int) int {
	return int(math.Max(float64(a), float64(b)))
}

func min(a int, b int) int {
	return int(math.Min(float64(a), float64(b)))
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
