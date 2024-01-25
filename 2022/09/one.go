package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type cord struct {
	x *int
	y *int
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
	x := 0
	y := 0
	t := cord{&x, &y}
	p := 0
	q := 0
	h := cord{&p, &q}

	for i, a := range ac {
		for j := 0; j < tss[i]; j++ {
			if a == "R" {
				*h.x += 1
				move(&h, &t)
			} else if a == "L" {
				*h.x -= 1
				move(&h, &t)
			} else if a == "U" {
				*h.y += 1
				move(&h, &t)
			} else if a == "D" {
				*h.y -= 1
				move(&h, &t)
			}
			tails[strconv.Itoa(*t.x)+":"+strconv.Itoa(*t.y)] = true
		}
	}

	fmt.Println(len(tails))
}

func move(h *cord, t *cord) {
	dx := *h.x - *t.x
	dy := *h.y - *t.y
	adx := abs(dx)
	ady := abs(dy)

	if adx == 2 && ady == 2 {
		*t.x += dx / 2
		*t.y += dy / 2
	} else if adx == 2 {
		*t.x += dx / 2
		*t.y = *h.y
	} else if ady == 2 {
		*t.x = *h.x
		*t.y += dy / 2
	}
}

func abs(n int) int {
	if n < 0 {
		return -1 * n
	}
	return n
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
