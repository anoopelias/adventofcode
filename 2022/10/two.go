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

	lines := make([][]string, 0)
	for i := 0; i < 6; i++ {
		line := make([]string, 0)
		for j := 0; j < 40; j++ {
			line = append(line, ".")
		}
		lines = append(lines, line)
	}

	cy := 0
	astn := 0
	ex := false
	xreg := 1
	for {
		j := cy % 40
		if j == xreg || j == (xreg+1) || j == (xreg-1) {
			i := cy / 40
			lines[i][j] = "#"
		}
		cy++
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

	for _, l := range lines {
		fmt.Println(strings.Join(l, " "))
	}
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
