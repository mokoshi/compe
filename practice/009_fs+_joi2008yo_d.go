// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Point struct {
	x int
	y int
}

func main() {
	m := readInt()
	a := make([]Point, m)
	for i := 0; i < m; i++ {
		a[i].x = readInt()
		a[i].y = readInt()
	}

	n := readInt()
	table := map[int]map[int]bool{}
	for i := 0; i < n; i++ {
		x := readInt()
		y := readInt()
		_, ok := table[x]
		if ok {
			table[x][y] = true
		} else {
			table[x] = map[int]bool{y: true}
		}
	}

	for x, ymap := range table {
		for y, _ := range ymap {
			completed := true
			dx := x - a[0].x
			dy := y - a[0].y
			for i := 1; i < m; i++ {
				ax := a[i].x + dx
				ay := a[i].y + dy
				_, ok := table[ax][ay]
				if !ok {
					completed = false
					break
				}
			}
			if completed {
				fmt.Println(dx, dy)
				return
			}
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
