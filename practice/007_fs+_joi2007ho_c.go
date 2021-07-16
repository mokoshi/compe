// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_c
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
	n := readInt()

	points := make([]Point, n)
	for i := 0; i < n; i++ {
		points[i].x = readInt()
		points[i].y = readInt()
	}

	table := map[int]map[int]int{}
	for i := 0; i < n-1; i++ {
		x := points[i].x
		y := points[i].y
		_, ok := table[x]
		if ok {
			table[x][y] = i
		} else {
			table[x] = map[int]int{y: i}
		}
	}

	max := 0
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			dx := points[j].x - points[i].x
			dy := points[j].y - points[i].y

			third1x := points[j].x - dy
			third1y := points[j].y + dx
			forth1x := third1x - dx
			forth1y := third1y - dy
			third2x := points[j].x + dy
			third2y := points[j].y - dx
			forth2x := third2x - dx
			forth2y := third2y - dy
			_, found1 := table[third1x][third1y]
			_, found2 := table[forth1x][forth1y]
			_, found3 := table[third2x][third2y]
			_, found4 := table[forth2x][forth2y]
			if (found1 && found2) || (found3 && found4) {
				size := dx*dx + dy*dy
				if max < size {
					max = size
				}
			}
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
