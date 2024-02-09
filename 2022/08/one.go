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
	ls := linesOf("../../../aoc-files/2022/08/input2")
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
	v := make([][]bool, 0)
	for i, row := range grid {
		vr := make([]bool, 0)
		for j, _ := range row {
			vc := false
			if i == len(grid)-1 || i == 0 || j == len(grid[0])-1 || j == 0 {
				vc = true
			}
			vr = append(vr, vc)
		}
		v = append(v, vr)
	}
	for i, _ := range grid {
		l := grid[i][0]
		for j := 1; j < len(grid[0]); j++ {
			if grid[i][j] > l {
				v[i][j] = true
				l = grid[i][j]
			}
		}

		l = grid[i][len(grid[0])-1]
		for j := len(grid[0]) - 2; j >= 0; j-- {
			if grid[i][j] > l {
				v[i][j] = true
				l = grid[i][j]
			}

		}

	}

	for j, _ := range grid[0] {
		l := grid[0][j]
		for i := 1; i < len(grid); i++ {
			if grid[i][j] > l {
				v[i][j] = true
				l = grid[i][j]
			}
		}
		l = grid[len(grid)-1][j]
		for i := len(grid) - 2; i >= 0; i-- {
			if grid[i][j] > l {
				v[i][j] = true
				l = grid[i][j]
			}
		}
	}

	cnt := 0

	for i, _ := range grid {
		for j, _ := range grid[0] {
			if v[i][j] {
				fmt.Print("t")

			} else {
				fmt.Print("f")

			}
		}
		fmt.Println()
	}

	for i, _ := range grid {
		for j, _ := range grid[0] {
			if v[i][j] {
				cnt++
			}
		}
	}
	fmt.Println(cnt)

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
