// https://atcoder.jp/contests/abc095/tasks/arc096_a
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	n := readInt()
	s := readString()

	count := 0
	for i := 0; i <= 9; i++ {
		for j := 0; j <= 9; j++ {
			for k := 0; k <= 9; k++ {
				ok1 := false
				ok2 := false
				for l := 0; l < n; l++ {
					num := int(s[l] - 48)
					if !ok1 {
						if num == i {
							ok1 = true
						}
					} else if !ok2 {
						if num == j {
							ok2 = true
						}
					} else if num == k {
						count++
						break
					}
				}
			}
		}
	}

	fmt.Println(count)
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
