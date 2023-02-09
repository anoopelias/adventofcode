package main

import (
	"fmt"
	goutils "go-utils"
	"strconv"
)

func fromSnafu(s string) int {
	pow := 1
	num := 0
	for i := len(s) - 1; i >= 0; i-- {
		num += fromSnafuDigit(s[i]) * pow
		pow *= 5
	}

	return num
}

func fromSnafuDigit(c byte) int {
	switch c {
	case '-':
		return -1
	case '=':
		return -2
	default:
		return int(c) - int('0')
	}
}

/*

0  0    0
1  1    1
2  2    2
3  5-2  1=
4  5-1  1-
5  5    10
6  5+1  11
7  5+2  12
8  10-2 2=
9  10-1 2-
10 10   20

18 20-2 4=
*/

func toSnafu(n int) string {
	snafu := ""
	for n != 0 {
		c, s := toSnafuDigit(n % 5)
		n -= c
		n /= 5
		snafu = s + snafu
	}
	return snafu
}

func toSnafuDigit(n int) (c int, s string) {
	switch n {
	case 3:
		c = -2
		s = "="
	case 4:
		c = -1
		s = "-"
	default:
		c = n
		s = strconv.Itoa(n)
	}
	return
}

func solve(ls []string) string {
	sum := 0
	for _, l := range ls {
		sum += fromSnafu(l)
	}
	return toSnafu(sum)
}

func main() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("input2")
	fmt.Println(solve(ls))
}
