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
	tsx := make([]int, 0)
	tsy := make([]int, 0)
	x := int(0)
	y := int(0)
	h := cord{&x, &y}
	t := make([]*cord, 0)

	for i := 0; i < 9; i++ {
		x := int(0)
		y := int(0)
		co := cord{&x, &y}
		t = append(t, &co)
	}

	for i, a := range ac {
		for j := 0; j < tss[i]; j++ {
			if a == "R" {
				*h.x += 1
			} else if a == "L" {
				*h.x -= 1
			} else if a == "U" {
				*h.y += 1
			} else if a == "D" {
				*h.y -= 1
			}
			move(&h, t[0])
			for i := 1; i < 9; i++ {
				move(t[i-1], t[i])
			}
			tsx = append(tsx, *t[8].x)
			tsy = append(tsy, *t[8].y)
			tails[strconv.Itoa(*t[8].x)+":"+strconv.Itoa(*t[8].y)] = true
		}
	}

	fmt.Println(len(tails))
	print(tsx, tsy)
}

func print(xs []int, ys []int) {
	minx := int(^uint(0) >> 1)
	maxx := -minx - 1

	miny := int(^uint(0) >> 1)
	maxy := -miny - 1

	for i := 0; i < len(xs); i++ {
		if xs[i] > maxx {
			maxx = xs[i]
		}
		if xs[i] < minx {
			minx = xs[i]
		}
		if ys[i] > maxy {
			maxy = ys[i]
		}
		if ys[i] < miny {
			miny = ys[i]
		}
	}

	grid := make([][]string, 0)
	for i := 0; i < (maxy - miny + 1); i++ {
		row := make([]string, 0)
		for j := 0; j < (maxx - minx + 1); j++ {
			row = append(row, " ")
		}
		grid = append(grid, row)
	}

	for i := 0; i < len(xs); i++ {
		grid[-ys[i]+maxy][xs[i]-minx] = "1"
	}
	grid[maxy][0-minx] = "s"
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			fmt.Print(grid[i][j] + " ")
		}
		fmt.Println()
	}

}

func move(h *cord, t *cord) {
	if *h.y == *t.y {
		if *h.x-*t.x == 2 {
			*t.x += 1
		} else if *h.x-*t.x == -2 {
			*t.x -= 1
		}
	} else if *h.x == *t.x {
		if *h.y-*t.y == 2 {
			*t.y += 1
		} else if *h.y-*t.y == -2 {
			*t.y -= 1
		}
	} else {
		if *h.x-*t.x == 2 {
			*t.x += 1
			*t.y = *h.y
		} else if *h.x-*t.x == -2 {
			*t.x -= 1
			*t.y = *h.y
		} else if *h.y-*t.y == 2 {
			*t.x = *h.x
			*t.y += 1
		} else if *h.y-*t.y == -2 {
			*t.x = *h.x
			*t.y -= 1
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
