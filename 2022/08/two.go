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

func main() {
	//fmt.Println("Starting...")
	ls := linesOf("input2")
	grid := make([][]int, 0)
	for _, line := range ls {
		rs := strings.Split(line, "")
		row := make([]int, 0)
		for _, cs := range rs {
			c, _ := strconv.Atoi(cs)
			row = append(row, c)
		}
		grid = append(grid, row)
	}
	max := 0

	for i, row := range grid {
		for j := range row {
			ss := up(grid, i, j) * down(grid, i, j) *
				left(grid, i, j) * right(grid, i, j)
			if ss > max {
				max = ss
			}

		}
	}
	fmt.Println(max)

}

func up(grid [][]int, p int, q int) int {
	l := grid[p][q]
	c := 0

	for i := p - 1; i >= 0; i-- {
		c++
		if grid[i][q] >= l {
			break
		}
	}
	return c
}

func down(grid [][]int, p int, q int) int {
	l := grid[p][q]
	c := 0

	for i := p + 1; i < len(grid); i++ {
		c++
		if grid[i][q] >= l {
			break
		}
	}
	return c
}

func left(grid [][]int, p int, q int) int {
	l := grid[p][q]
	c := 0

	for j := q - 1; j >= 0; j-- {
		c++
		if grid[p][j] >= l {
			break
		}
	}
	return c
}

func right(grid [][]int, p int, q int) int {
	l := grid[p][q]
	c := 0

	for j := q + 1; j < len(grid[0]); j++ {
		c++
		if grid[p][j] >= l {
			break
		}
	}
	return c
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
