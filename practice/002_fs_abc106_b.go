// https://atcoder.jp/contests/abc106/tasks/abc106_b
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	n := readInt()

	ans := 0
	for i := 1; i <= (n+1)/2; i++ {
		divisorNum := getDivisorNum(i*2 - 1)
		if divisorNum == 8 {
			ans++
		}
	}

	fmt.Println(ans)
}

func getDivisorNum(n int) int {
	num := 0
	max := int(math.Sqrt(float64(n)))
	for i := 1; i <= max; i++ {
		if i*i == n {
			num++
			break
		} else if n%i == 0 {
			num += 2
		}
	}
	return num
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
