package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	n := readInt()

	c := make([]int, n)
	for i := 0; i < n; i++ {
		c[i] = readInt()
	}
	sort.Ints(c)

	d := uint64(1e9 + 7)
	var ans uint64 = 1
	for i := 0; i < n; i++ {
		p := uint64(c[i] - i)
		if p <= 0 {
			fmt.Println(0)
			return
		}

		ans = (ans * p) % d
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
