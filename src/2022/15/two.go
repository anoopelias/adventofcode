package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

type coord struct {
	x int
	y int
}

type senbeac struct {
	s coord
	b coord
}

type transgrid struct {
	minx int
	miny int
	grid *[]*[]string
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	sbs := make([]senbeac, 0)
	minx, miny := math.MaxInt, math.MaxInt
	maxx, maxy := 0, 0
	for i := 0; i < len(ls); i++ {
		rx, _ := regexp.Compile("Sensor at x=([-]?[0-9]+), y=([-]?[0-9]+): closest beacon is at x=([-]?[0-9]+), y=([-]?[0-9]+)")
		cs := rx.FindStringSubmatch(ls[i])
		sx, _ := strconv.Atoi(cs[1])
		sy, _ := strconv.Atoi(cs[2])
		bx, _ := strconv.Atoi(cs[3])
		by, _ := strconv.Atoi(cs[4])

		sb := senbeac{
			s: coord{sx, sy},
			b: coord{bx, by},
		}

		minx = minOf(minx, sx)
		minx = minOf(minx, bx)

		miny = minOf(miny, sy)
		miny = minOf(miny, by)

		maxx = maxOf(maxx, sx)
		maxx = maxOf(maxx, bx)

		maxy = maxOf(maxy, sy)
		maxy = maxOf(maxy, by)

		sbs = append(sbs, sb)
	}

	tg := newGrid(minx, miny, maxx, maxy)

	for _, sb := range sbs {
		tg.set(sb.s, "S")
		tg.set(sb.b, "B")
		md := mandist(sb.s, sb.b)
		tg.setScanner(sb.s, md)
	}

	tg.print(0, 4000000)
	fmt.Println(tg.counthash(10))
}

func newGrid(minx, miny, maxx, maxy int) transgrid {
	grid := make([]*[]string, 0)
	for i := 0; i < (maxy-miny)+1; i++ {
		row := make([]string, 0)
		for j := 0; j < (maxx-minx)+1; j++ {
			row = append(row, ".")
		}
		grid = append(grid, &row)
	}
	return transgrid{
		minx: minx,
		miny: miny,
		grid: &grid,
	}
}

func (tg *transgrid) print(min, max int) {
	zx := min - tg.minx
	zy := min - tg.miny

	for i := zy; i < zy+(max-min); i++ {
		fmt.Printf("%2d ", i-zy)
		for j := zx; j < zx+(max-min); j++ {
			fmt.Print((*(*tg.grid)[i])[j], " ")
		}
		fmt.Println()
	}
}

func (tg *transgrid) counthash(n int) int {
	row := (*(*tg.grid)[n])

	cnt := 0
	for _, s := range row {
		if s == "#" {
			cnt++
		}
	}
	return cnt
}

func (tg *transgrid) setScanner(s coord, md int) {
	for i := s.y - md; i <= s.y+md; i++ {
		tg.setRow(i, s.x, md-abs(s.y-i))
	}

}

func (tg *transgrid) setRow(r int, c int, xdist int) {
	for i := c - xdist; i <= c+xdist; i++ {
		co := coord{
			x: i,
			y: r,
		}
		tg.set(co, "#")
	}
}

func (tg *transgrid) set(c coord, s string) {
	x := c.x - tg.minx
	y := c.y - tg.miny

	if x < 0 || x >= len(*(*tg.grid)[0]) || y < 0 || y >= len(*tg.grid) {
		return
	}

	if (*(*tg.grid)[y])[x] == "B" || (*(*tg.grid)[y])[x] == "S" {
		return
	}

	(*(*tg.grid)[y])[x] = s
}

func mandist(c1, c2 coord) int {
	return abs(c1.x-c2.x) + abs(c1.y-c2.y)
}

func abs(x int) int {
	if x < 0 {
		return x * -1
	}
	return x
}

func minOf(min, x int) int {
	if x < min {
		return x
	}
	return min
}

func maxOf(max, x int) int {
	if x > max {
		return x
	}
	return max
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
