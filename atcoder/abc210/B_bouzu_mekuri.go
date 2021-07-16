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

	for i := 0; i < n; i++ {
		if s[i] == '1' {
			if i % 2 == 0 {
				fmt.Println("Takahashi")
			} else {
				fmt.Println("Aoki")
			}
			break
		}
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
