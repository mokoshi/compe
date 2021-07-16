// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_B&lang=ja
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	for true {
		n := readInt()
		x := readInt()
		if n == 0 && x == 0 {
			break
		}

		ans := 0
		for i := 1; i <= n-2; i++ {
			for j := i + 1; j <= n-1; j++ {
				for k := j + 1; k <= n; k++ {
					if i+j+k == x {
						ans++
					}
				}
			}
		}
		fmt.Println(ans)
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
