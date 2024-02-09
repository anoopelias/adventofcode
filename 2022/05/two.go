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
	ls := linesOf("../../../aoc-files/2022/05/input2")

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))
	stacks := make([][]string, 0)
	for _, l := range ls {
		if len(strings.Trim(l, " ")) > 0 {
			if strings.Trim(l, " ")[0] == '[' {
				l = l + " "
				for i := 0; i < (len(l)+1)/4; i++ {
					ch := l[i*4 : i*4+4][1]
					if len(stacks) == i {
						stacks = append(stacks, []string{})
					}
					if ch != ' ' {
						stacks[i] = append(stacks[i], string(ch))
					}
				}
			} else if strings.Trim(l, " ")[0] == '1' {
				// do nothing
			} else {
				rx, _ := regexp.Compile("move ([0-9]+) from ([0-9]+) to ([0-9]+)")
				ms := rx.FindStringSubmatch(l)
				n, _ := strconv.Atoi(ms[1])
				f, _ := strconv.Atoi(ms[2])
				t, _ := strconv.Atoi(ms[3])
				f--
				t--

				st := append([]string{}, stacks[f][0:n]...)
				stacks[t] = append(st, stacks[t]...)
				stacks[f] = stacks[f][n:]
			}
		}
	}

	ret := ""
	for _, st := range stacks {
		ret += st[0]
	}

	fmt.Println(ret)

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
