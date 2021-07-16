// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_13_A&lang=ja
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	k := readInt()
	r := make([]int, k)
	c := make([]int, k)
	for i := 0; i < k; i++ {
		r[i] = readInt()
		c[i] = readInt()
	}

	p := make([]int, 8)
	for i := 0; i < 8; i++ {
		p[i] = i
	}

	for ok := true; ok; ok = nextPermutation(p) {
		flag := true
		for i := 0; i < k; i++ {
			if p[r[i]] != c[i] {
				flag = false
				break
			}
		}
		if !flag {
			continue
		}

		upLines := map[int]bool{}
		downLines := map[int]bool{}
		for i := -7; i <= 7; i++ {
			upLines[i] = false
			downLines[i] = false
		}

		satis := true
		for x := 0; x < 8; x++ {
			y := p[x]
			if downLines[x-y] {
				satis = false
				break
			} else {
				downLines[x-y] = true
			}

			if upLines[x+y-7] {
				satis = false
				break
			} else {
				upLines[x+y-7] = true
			}
		}

		if satis {
			for x := 0; x < 8; x++ {
				line := ""
				for y := 0; y < 8; y++ {
					if p[x] == y {
						line += "Q"
					} else {
						line += "."
					}
				}
				fmt.Println(line)
			}
			return
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

func nextPermutation(a []int) bool {
	return permutationPandita(a, func(x, y int) bool { return x < y })
}

func prevPermutation(a []int) bool {
	return permutationPandita(a, func(x, y int) bool { return y < x })
}

func permutationPandita(a []int, less func(x, y int) bool) bool {
	i := len(a) - 2
	for 0 <= i && !less(a[i], a[i+1]) {
		i--
	}
	if i == -1 {
		return false
	}
	j := i + 1
	for k := j + 1; k < len(a); k++ {
		if less(a[i], a[k]) && !less(a[j], a[k]) {
			j = k
		}
	}
	a[i], a[j] = a[j], a[i]
	for p, q := i+1, len(a)-1; p < q; p, q = p+1, q-1 {
		a[p], a[q] = a[q], a[p]
	}
	return true
}
