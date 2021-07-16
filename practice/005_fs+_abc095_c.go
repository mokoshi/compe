// https://atcoder.jp/contests/abc095/tasks/arc096_a
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	a := readInt()
	b := readInt()
	c := readInt()
	x := readInt()
	y := readInt()

	if (a+b)/2 < c {
		fmt.Println(a*x + b*y)
	} else {
		cNum := int(math.Min(float64(x), float64(y)))
		aNum := x - cNum
		bNum := y - cNum

		cost := cNum * 2 * c
		if aNum > 0 && a > c*2 {
			cost += aNum * 2 * c
		} else if bNum > 0 && b > c*2 {
			cost += bNum * 2 * c
		} else {
			cost += aNum*a + bNum*b
		}
		fmt.Println(cost)
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
