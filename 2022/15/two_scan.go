package main

import (
	"fmt"
	"io/ioutil"
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
	n   int
	max int
	row *[]bool
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/15/input2")

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	sbs := make([]senbeac, 0)
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

		sbs = append(sbs, sb)
	}

	//max := 20
	max := 4000000
	for i := 0; i <= max; i++ {
		tg := newGrid(i, 20)
		for _, sb := range sbs {
			tg.set(sb.s)
			tg.set(sb.b)
			md := mandist(sb.s, sb.b)
			tg.setScanner(sb.s, md)
		}

		tf := tg.tuningFreq()
		if tf != -1 {
			fmt.Println(tf)
			break
		}

		if i%100 == 0 {
			fmt.Println(i)
		}
	}
}

func newGrid(n int, max int) transgrid {
	row := make([]bool, 0)
	for i := 0; i <= max; i++ {
		row = append(row, false)
	}
	return transgrid{
		row: &row,
		max: max,
		n:   n,
	}
}

func (tg *transgrid) tuningFreq() int {

	for i, f := range *tg.row {
		if !f {
			fmt.Printf("Found tf %d, %d\n", i, tg.n)
			return i*4000000 + tg.n
		}
	}
	return -1
}

func (tg *transgrid) setScanner(s coord, md int) {
	if abs(tg.n-s.y) <= md {
		tg.setRow(tg.n, s.x, md-abs(s.y-tg.n))
	}
}

func (tg *transgrid) setRow(r int, c int, xdist int) {
	for i := c - xdist; i <= c+xdist; i++ {
		co := coord{
			x: i,
			y: r,
		}
		tg.set(co)
	}
}

func (tg *transgrid) set(c coord) {

	if c.y != tg.n {
		return
	}

	if c.x < 0 || c.x > tg.max {
		return
	}
	(*tg.row)[c.x] = true
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
