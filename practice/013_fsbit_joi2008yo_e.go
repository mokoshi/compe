// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	r := readInt()
	c := readInt()

	table := make([][]int, r)
	for i := 0; i < r; i++ {
		table[i] = make([]int, c)
		for j := 0; j < c; j++ {
			b := readInt()
			table[i][j] = b
		}
	}

	ans := 0
	patterns := 1 << r
	for p := 0; p < patterns; p++ {
		total := 0
		for i := 0; i < c; i++ {
			column := 0
			for j := 0; j < r; j++ {
				if ((p >> j) & 1) == 1 {
					column += 1 - table[j][i]
				} else {
					column += table[j][i]
				}
			}
			if column > r-column {
				total += column
			} else {
				total += r - column
			}
		}
		if ans < total {
			ans = total
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
