package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type node struct {
	p      int
	q      int
	dist   int
	c      byte
	marked bool
	ns     []*node
	end    bool
}

type coord struct {
	x int
	y int
}

type ndist struct {
	n *node
	d int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/12/input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	grid := [][]*node{}
	var root *node

	for i := 0; i < len(ls); i++ {
		row := []*node{}
		for j := 0; j < len(ls[i]); j++ {
			row = append(row, &node{
				p: i,
				q: j,
				c: ls[i][j],
			})
			if row[j].c == 'E' {
				row[j].c = 'z'
				row[j].end = true
			}
			if row[j].c == 'S' {
				row[j].c = 'a'
				root = row[j]
			}
		}
		grid = append(grid, row)
	}
	for i := 0; i < len(ls); i++ {
		for j := 0; j < len(ls[i]); j++ {
			grid[i][j].ns = neighbors(grid, i, j)
		}
	}

	// bfs
	qu := []ndist{{root, -1}}
	for len(qu) != 0 {
		h, nqu := qu[0], qu[1:]
		qu = nqu
		if !h.n.marked {
			h.n.marked = true
			h.n.dist = h.d + 1
			for _, ne := range h.n.ns {
				qu = append(qu, ndist{ne, h.n.dist})
			}

			if h.n.end {
				fmt.Println("dist", h.n.dist)
				break
			}
		}
	}
}

func neighbors(grid [][]*node, p int, q int) []*node {
	ns := make([]*node, 0)
	npq := []coord{
		{p + 1, q},
		{p - 1, q},
		{p, q + 1},
		{p, q - 1},
	}

	for _, co := range npq {
		if co.x >= 0 && co.y >= 0 && co.x < len(grid) && co.y < len(grid[0]) {
			if grid[p][q].c >= grid[co.x][co.y].c ||
				grid[co.x][co.y].c-grid[p][q].c == 1 {

				ns = append(ns, grid[co.x][co.y])
			}
		}
	}
	return ns
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
