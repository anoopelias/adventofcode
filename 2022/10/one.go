package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type smt struct {
	c string
	n int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	ast := make([]smt, 0)
	for _, l := range ls {
		sps := strings.Split(l, " ")
		if len(sps) == 1 {
			ast = append(ast, smt{c: "noop"})
		} else {
			n, _ := strconv.Atoi(sps[1])
			ast = append(ast, smt{c: "addx", n: n})
		}
	}

	cy := 0
	astn := 0
	ex := false
	xreg := 1
	ss := 0
	for {
		cy++
		if (cy-20)%40 == 0 {
			ss += cy * xreg
		}
		if ex {
			xreg += ast[astn].n
			astn++
			ex = false
			if astn == len(ast) {
				break
			}
		} else if ast[astn].c == "noop" {
			astn++
			if astn == len(ast) {
				break
			}
		} else {
			ex = true
		}
	}
	fmt.Println(ss)
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
