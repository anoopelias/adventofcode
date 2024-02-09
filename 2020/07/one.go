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
	ls := linesOf("../../../aoc-files/2020/07/input2")
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
				if _, f := g[cbag]; !f {
					g[cbag] = make(map[string]int)
				}
				g[cbag][bag] = 0
			}
		}
	}

	bags := keysOf(g)
	marked := make([]bool, len(bags))
	queue := []string{"shiny gold"}
	var head string

	for len(queue) != 0 {
		head, queue = queue[0], queue[1:]
		if !marked[findBag(head, bags)] {
			cts := g[head]
			marked[findBag(head, bags)] = true
			for _, ct := range keysOfCts(cts) {
				if !marked[findBag(ct, bags)] {
					queue = append(queue, ct)
				}
			}
		}
	}
	cnt := 0
	for _, b := range marked {
		if b {
			cnt++
		}
	}

	fmt.Println(cnt - 1)
}

func findBag(b string, bags []string) int {
	for i, bag := range bags {
		if bag == b {
			return i
		}
	}
	return -1
}

func keysOf(m map[string]map[string]int) []string {
	keys := make([]string, len(m))

	i := 0
	for k := range m {
		keys[i] = k
		i++
	}
	return keys
}

func keysOfCts(m map[string]int) []string {
	keys := make([]string, len(m))

	i := 0
	for k := range m {
		keys[i] = k
		i++
	}
	return keys
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
