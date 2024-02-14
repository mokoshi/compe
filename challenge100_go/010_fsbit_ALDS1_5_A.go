// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A&lang=ja
// bit探索だとTLE
// 再帰で探索して、不要なルートは途中で打ち切るようにしないとだめだった
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	a := make([]int, n)
	for i := 0; i < n; i++ {
		a[i] = readInt()
	}

	var solve func(i int, m int) bool
	solve = func(i int, m int) bool {
		if m == 0 {
			return true
		} else if m < 0 || i == n {
			return false
		}
		return solve(i+1, m-a[i]) || solve(i+1, m)
	}

	q := readInt()
	for i := 0; i < q; i++ {
		m := readInt()

		if solve(0, m) {
			fmt.Println("yes")
		} else {
			fmt.Println("no")
		}
	}
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
