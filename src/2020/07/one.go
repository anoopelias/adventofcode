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
	rx, _ := regexp.Compile("([0-9]*) (.*?) bag(s)?(, ([0-9])* (.*) bag(s)?)?.")
	for _, l := range ls {
		splits := strings.Split(l, "bags contain")
		bag := strings.Trim(splits[0], " ")

		if _, f := g[bag]; !f {
			g[bag] = make([]string, 0)
		}

		if !strings.Contains(splits[1], "no other bags") {
			bag1 := rx.FindStringSubmatch(splits[1])[1]
			bag2 := rx.FindStringSubmatch(splits[1])[3]
			if _, f := g[bag1]; !f {
				g[bag1] = make([]string, 0)
			}
			if _, f := g[bag2]; !f {
				g[bag2] = make([]string, 0)
			}
			if len(bag1) != 0 {
				g[bag1] = []string{bag}
			}
			if len(bag2) != 0 {
				g[bag2] = []string{bag}
			}
		}
	}

	bags := keysOf(g)
	marked := make([]bool, len(bags))
	queue := make([]string, 0)
	queue = append(queue, "shiny gold")
	marked[findBag("shiny gold", bags)] = true
	var head string

	for len(queue) != 0 {
		head, queue = queue[0], queue[1:]
		if !marked[findBag(head, bags)] {
			covers := g[head]
			marked[findBag(head, bags)] = true
			for _, cover := range covers {
				if !marked[findBag(cover, bags)] {
					queue = append(queue, cover)
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

	fmt.Println(cnt)
}

func findBag(b string, bags []string) int {
	for i, bag := range bags {
		if bag == b {
			return i
		}
	}
	return -1
}

func keysOf(m map[string][]string) []string {
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
