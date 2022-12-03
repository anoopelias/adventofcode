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
	ls := linesOf("input")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	g := make(map[string][]string)
	rx, _ := regexp.Compile("([0-9]*) ([a-zA-Z ]*) bag(s)?(, ([0-9])* (.*)bag(s)?)?.")
	for _, l := range ls {
		splits := strings.Split(l, "bags contain")
		bag := strings.Trim(splits[0], " ")

		if _, f := g[bag]; !f {
			g[bag] = make([]string, 0)
		}

		if !strings.Contains(splits[1], "no other bags") {
			bag1 := rx.FindStringSubmatch(splits[1])[2]
			bag2 := rx.FindStringSubmatch(splits[1])[6]
			if _, f := g[bag1]; !f {
				g[bag1] = make([]string, 0)
			}
			if _, f := g[bag2]; !f {
				g[bag2] = make([]string, 0)
			}
		}
	}
}

func setEdge()

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
