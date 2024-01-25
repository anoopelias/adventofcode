package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	g := make(map[string]map[string]int)
	rx, _ := regexp.Compile("([0-9]*) ([a-z ]*) bag(s)?\\.?")
	for _, l := range ls {
		splits := strings.Split(l, "bags contain")
		bag := strings.Trim(splits[0], " ")

		if _, f := g[bag]; !f {
			g[bag] = make(map[string]int)
		}

		if !strings.Contains(splits[1], "no other bags") {
			cts := strings.Split(splits[1], ",")
			for _, ct := range cts {
				cbag := rx.FindStringSubmatch(ct)[2]
				cn, _ := strconv.Atoi(rx.FindStringSubmatch(ct)[1])
				if _, f := g[cbag]; !f {
					g[cbag] = make(map[string]int)
				}
				g[bag][cbag] = cn
			}
		}
	}

	fmt.Println(numberOfBagsIn(g, "shiny gold"))
}

func numberOfBagsIn(g map[string]map[string]int, bag string) int {
	c, _ := g[bag]
	r := 0
	for b, n := range c {
		ni := numberOfBagsIn(g, b)
		r += n + n*ni
	}
	return r
}

func findBag(b string, bags []string) int {
	for i, bag := range bags {
		if bag == b {
			return i
		}
	}
	return -1
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
