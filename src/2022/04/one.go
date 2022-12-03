package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input")
	p := 0

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))
	for _, l := range ls {
		fmt.Println(l)
	}

	fmt.Println(p)
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
