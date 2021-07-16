package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	k := readInt()

	c := make([]int, n)
	for i := 0; i < n; i++ {
		c[i] = readInt()
	}

	table := map[int]int{}
	for i := 0; i < k; i++ {
		table[c[i]] ++
	}
	ans := len(table)

	for i := k; i < n; i++ {
		rem := table[c[i-k]]
		if rem == 1 {
			delete(table, c[i-k])
		} else {
			table[c[i-k]] --
		}
		table[c[i]] ++

		current := len(table)
		if ans < current {
			ans = current
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
