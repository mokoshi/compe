// https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b
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
	for i := 0; i < n; i++ {
		a[i] = readInt()
		b[i] = readInt()
	}

	sort.Ints(a)
	sort.Ints(b)

	start := a[len(a)/2]
	end := b[len(b)/2]

	var cost int64 = 0
	for i := 0; i < n; i++ {
		cost += int64(math.Abs(float64(start-a[i])) + math.Abs(float64(a[i]-b[i])) + math.Abs(float64(b[i]-end)))
	}

	fmt.Println(cost)
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
