package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Dir struct {
	name string
	dirs []*Dir
	file []int
	p    *Dir
}

type cord struct {
	x int
	y int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	ac := make([]string, 0)
	tss := make([]int, 0)
	for _, line := range ls {
		sps := strings.Split(line, " ")
		ts, _ := strconv.Atoi(sps[1])
		ac = append(ac, sps[0])
		tss = append(tss, ts)
	}

	tails := make(map[string]bool)
	t := cord{0, 0}
	h := cord{0, 0}

	for i, a := range ac {
		for j := 0; j < tss[i]; j++ {
			if a == "R" {
				h.x += 1
				move(&h, &t)
			} else if a == "L" {
				h.x -= 1
				move(&h, &t)
			} else if a == "U" {
				h.y += 1
				move(&h, &t)
			} else if a == "D" {
				h.y -= 1
				move(&h, &t)
			}
			tails[strconv.Itoa(t.x)+":"+strconv.Itoa(t.y)] = true
		}
	}

	fmt.Println(len(tails))
}

func move(h *cord, t *cord) {
	if h.y == t.y {
		if h.x-t.x == 2 {
			t.x += 1
		} else if h.x-t.x == -2 {
			t.x -= 1
		}
	} else if h.x == t.x {
		if h.y-t.y == 2 {
			t.y += 1
		} else if h.y-t.y == -2 {
			t.y -= 1
		}
	} else {
		if h.x-t.x == 2 {
			t.x += 1
			t.y = h.y
		} else if h.x-t.x == -2 {
			t.x -= 1
			t.y = h.y
		} else if h.y-t.y == 2 {
			t.x = h.x
			t.y += 1
		} else if h.y-t.y == -2 {
			t.x = h.x
			t.y -= 1
		}
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
