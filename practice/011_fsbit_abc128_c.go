// https://atcoder.jp/contests/abc128/tasks/abc128_c
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

	s := make([][]int, m)
	for i := 0; i < m; i++ {
		k := readInt()
		s[i] = make([]int, k)
		for j := 0; j < k; j++ {
			s[i][j] = readInt()
		}
	}

	p := make([]int, m)
	for i := 0; i < m; i++ {
		p[i] = readInt()
	}

	ans := 0
	patterns := 1 << uint(n)
	for i := 0; i < patterns; i++ {
		ok := true
		for j := 0; j < m; j++ {
			switchCount := 0
			for k := 0; k < len(s[j]); k++ {
				if (i>>(s[j][k]-1))&1 == 1 {
					switchCount++
				}
			}
			if switchCount%2 != p[j] {
				ok = false
				break
			}
		}
		if ok {
			ans++
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
