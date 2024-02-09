package main

import (
	"fmt"
	goutils "go-utils"
	"math"
	"strconv"
	"strings"
)

const (
	north int = iota
	south
	west
	east
)

type coord struct {
	top, left int
}

func solve(ls []string) string {
	m := make([][]string, 0)
	dir := north

	for _, l := range ls {
		m = append(m, strings.Split(l, ""))
	}

	i := 1
	for t, r := range m {
		for l, c := range r {
			if c == "#" {
				m[t][l] = strconv.Itoa(i)
				i++
			}
		}
	}

	// printMap(m)
	mvd := true
	cnt := 0
	for mvd {
		m, mvd = round(m, dir)
		dir = nextDir(dir)
		cnt++
	}
	// printMap(m)
	return strconv.Itoa(cnt)
}

func countDots(m [][]string) int {
	minT := math.MaxInt
	maxT := 0

	minL := math.MaxInt
	maxL := 0

	for t, r := range m {
		for l, c := range r {
			if c != "." {
				minT = goutils.Min(t, minT)
				maxT = goutils.Max(t, maxT)
				minL = goutils.Min(l, minL)
				maxL = goutils.Max(l, maxL)
			}
		}
	}

	cnt := 0
	for t := minT; t <= maxT; t++ {
		for l := minL; l <= maxL; l++ {
			if m[t][l] == "." {
				cnt++
			}
		}
	}
	return cnt
}

func printMap(m [][]string) {
	for _, r := range m {
		fmt.Println(strings.Join(r, "\t"))
	}
	fmt.Println()
}

func round(m [][]string, dir int) ([][]string, bool) {
	ps := make(map[coord]bool)
	ts := make(map[coord]coord)

	m = addEdges(m)
	for t, r := range m {
		for l, c := range r {
			if c != "." {
				co := coord{t, l}
				p := proposal(m, co, dir)
				if p != nil {
					if _, ok := ps[*p]; ok {
						ps[*p] = false
					} else {
						ps[*p] = true
						ts[co] = *p
					}
				}
			}
		}
	}

	mvd := false
	for k, v := range ts {
		if f, ok := ps[v]; ok && f {
			m[v.top][v.left] = m[k.top][k.left]
			m[k.top][k.left] = "."
			mvd = true
		}
	}
	return m, mvd
}

func isEmpty(m [][]string, cs []coord) bool {
	for _, c := range cs {
		if m[c.top][c.left] != "." {
			return false
		}
	}
	return true
}

func canMove(m [][]string, p coord) bool {
	return !isEmpty(m, neighbors(p))
}

func proposal(m [][]string, p coord, dir int) *coord {
	if !canMove(m, p) {
		return nil
	}
	for i := 0; i < 4; i++ {
		if isEmpty(m, posFor(p, dir)) {
			return next(p, dir)
		}
		dir = nextDir(dir)
	}
	return nil
}

func nextDir(dir int) int {
	if dir == east {
		return north
	}
	return dir + 1
}

func neighbors(p coord) []coord {
	return []coord{
		{p.top - 1, p.left - 1},
		{p.top - 1, p.left},
		{p.top - 1, p.left + 1},
		{p.top, p.left - 1},
		{p.top, p.left + 1},
		{p.top + 1, p.left - 1},
		{p.top + 1, p.left},
		{p.top + 1, p.left + 1},
	}
}

func next(p coord, dir int) *coord {
	var cs coord
	switch dir {
	case north:
		cs = coord{p.top - 1, p.left}
	case south:
		cs = coord{p.top + 1, p.left}
	case west:
		cs = coord{p.top, p.left - 1}
	case east:
		cs = coord{p.top, p.left + 1}
	}
	return &cs
}

func posFor(p coord, dir int) []coord {
	var cs []coord
	switch dir {
	case north:
		cs = []coord{
			{p.top - 1, p.left - 1},
			{p.top - 1, p.left},
			{p.top - 1, p.left + 1},
		}
	case south:
		cs = []coord{
			{p.top + 1, p.left - 1},
			{p.top + 1, p.left},
			{p.top + 1, p.left + 1},
		}
	case west:
		cs = []coord{
			{p.top - 1, p.left - 1},
			{p.top, p.left - 1},
			{p.top + 1, p.left - 1},
		}
	case east:
		cs = []coord{
			{p.top - 1, p.left + 1},
			{p.top, p.left + 1},
			{p.top + 1, p.left + 1},
		}
	}
	return cs
}

func addEdges(m [][]string) [][]string {
	m = addLeft(m)
	m = addRight(m)
	m = addTop(m)
	m = addBottom(m)
	return m
}

func addLeft(m [][]string) [][]string {
	nm := make([][]string, 0)
	ed := false
	for _, r := range m {
		if r[0] != "." {
			ed = true
			break
		}
	}
	if !ed {
		return m
	}
	for _, r := range m {
		nm = append(nm, append([]string{"."}, r...))
	}
	return nm
}

func addRight(m [][]string) [][]string {
	nm := make([][]string, 0)
	ed := false
	for _, r := range m {
		if r[len(r)-1] != "." {
			ed = true
			break
		}
	}
	if !ed {
		return m
	}
	for _, r := range m {
		nm = append(nm, append(r, "."))
	}
	return nm
}

func addTop(m [][]string) [][]string {
	nr := make([]string, 0)
	ed := false
	for _, c := range m[0] {
		if c != "." {
			ed = true
			break
		}
	}
	if !ed {
		return m
	}
	for i := 0; i < len(m[0]); i++ {
		nr = append(nr, ".")
	}
	return append([][]string{nr}, m...)
}

func addBottom(m [][]string) [][]string {
	nr := make([]string, 0)
	ed := false
	for _, c := range m[len(m)-1] {
		if c != "." {
			ed = true
			break
		}
	}
	if !ed {
		return m
	}
	for i := 0; i < len(m[0]); i++ {
		nr = append(nr, ".")
	}
	return append(m, nr)
}

func main() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("../../../aoc-files/2022/23/input2")
	fmt.Println(solve(ls))
}
