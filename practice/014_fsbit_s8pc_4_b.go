// https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_b
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
	k := readInt()

	a := make([]int64, n)
	for i := 0; i < n; i++ {
		a[i] = readInt64()
	}

	var ans int64 = math.MaxInt64
	patterns := 1 << (n - 1)
	for p := 0; p < patterns; p++ {
		var indices = []int{0}
		for i := 1; i < n; i++ {
			if p>>(i-1)&1 == 1 {
				indices = append(indices, i)
			}
		}
		if len(indices) != k {
			continue
		}

		result := make([]int64, len(indices))
		for i := 0; i < len(indices)-1; i++ {
			result[i] = a[indices[i]]
		}
		for i := 0; i < len(indices)-1; i++ {
			from := indices[i]
			to := indices[i+1]
			for j := from + 1; j <= to-1; j++ {
				h := a[j]
				if h > result[i] {
					result[i] = h
				}
			}
			if result[i+1] <= result[i] {
				result[i+1] = result[i] + 1
			}
		}

		var cost int64 = 0
		for i := 0; i < len(indices); i++ {
			cost += result[i] - a[indices[i]]
		}

		if cost < ans {
			ans = cost
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
