package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/04/input2")
	p := 0

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))
	for _, l := range ls {
		//fmt.Println(l)
		splits := strings.Split(l, ",")
		e1r := strings.Split(splits[0], "-")
		e2r := strings.Split(splits[1], "-")
		e1s, _ := strconv.Atoi(e1r[0])
		e1e, _ := strconv.Atoi(e1r[1])
		e2s, _ := strconv.Atoi(e2r[0])
		e2e, _ := strconv.Atoi(e2r[1])

		if (e1e < e2s) || (e2e < e1s) {
			p++
		}
	}

	fmt.Println(len(ls) - p)
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
