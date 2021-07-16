// https://atcoder.jp/contests/abc122/tasks/abc122_b
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	s := readString()

	ans := 0
	for i := 0; i < len(s); i++ {
		current := 0
		for j := i; j < len(s); j++ {
			c := s[j]
			if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
				current++
			} else {
				break
			}
		}
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
