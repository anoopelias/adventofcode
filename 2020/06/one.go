package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	s := 0

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))
	gr := make(map[rune]int)
	gf := true
	for _, l := range ls {
		//fmt.Println(l)
		if len(l) == 0 {
			s += len(gr)
			gr = make(map[rune]int)
			gf = true
		} else {
			lm := make(map[rune]int)
			for _, c := range l {
				lm[c] = 0
			}
			if !gf {
				gr = intersection(gr, lm)
			} else {
				gr = lm
			}
			gf = false
		}
	}

	fmt.Println(s)
}

func intersection(r1 map[rune]int, r2 map[rune]int) map[rune]int {
	r := make(map[rune]int)
	for k, _ := range r1 {
		if _, f := r2[k]; f {
			r[k] = 0
		}
	}
	return r
}

func linesOf(fn string) []string {
	fbyts, err := ioutil.ReadFile(fn)
	if err != nil {
		panic(err)
	}
	lines := make([]string, 0)
	for _, line := range strings.Split(string(fbyts), "\n") {
		lines = append(lines, line)

	}

	return lines
}
