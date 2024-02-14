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
	d := readInt()
	n := readInt()
	m := readInt()

	di := make([]int, n)
	di[0] = 0
	for i := 1; i < n; i++ {
		di[i] = readInt()
	}
	k := make([]int, m)
	for i := 0; i < m; i++ {
		k[i] = readInt()
	}
	sort.Ints(di)
	sort.Ints(k)

	distance := func(a int, b int) int {
		return min(abs(a-b), d-abs(a-b))
	}

	var closestDistance func([]int, int) int
	closestDistance = func(ss []int, f int) int {
		if len(ss) == 1 {
			return distance(ss[0], f)
		}
		left := ss[:len(ss)/2]
		right := ss[len(ss)/2:]

		leftMin := min(distance(left[0], f), distance(left[len(left)-1], f))
		rightMin := min(distance(right[0], f), distance(right[len(right)-1], f))
		if leftMin < rightMin {
			return closestDistance(left, f)
		} else {
			return closestDistance(right, f)
		}
	}

	ans := 0
	for i := 0; i < m; i++ {
		ans += closestDistance(di, k[i])
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
