// https://atcoder.jp/contests/abc002/tasks/abc002_4
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	m := readInt()

	table := map[int]map[int]bool{}
	for i := 0; i < m; i++ {
		x := readInt()
		y := readInt()

		_, exists := table[x]
		if exists {
			table[x][y] = true
		} else {
			table[x] = map[int]bool{y: true}
		}
	}

	max := 1
	patterns := 1 << n
	for i := 0; i < patterns; i++ {
		var members []int
		for j := 0; j < n; j++ {
			if ((i >> j) & 1) == 1 {
				members = append(members, j+1)
			}
		}
		size := len(members)
		flag := func() bool {
			for x := 0; x < size-1; x++ {
				for y := x + 1; y < size; y++ {
					if !table[members[x]][members[y]] {
						return false
					}
				}
			}
			return true
		}()

		if flag && size > max {
			max = size
		}
	}

	fmt.Println(max)
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
