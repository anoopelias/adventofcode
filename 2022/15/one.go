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
	n    int
	rows *map[int]bool
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")

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

	tg := newGrid(2000000)

	for _, sb := range sbs {
		tg.unset(sb.s)
		tg.unset(sb.b)
		md := mandist(sb.s, sb.b)
		tg.setScanner(sb.s, md)
	}

	fmt.Println(tg.counthash())
}

func newGrid(n int) transgrid {
	rows := make(map[int]bool)
	return transgrid{
		rows: &rows,
		n:    n,
	}
}

func (tg *transgrid) counthash() int {
	rows := *tg.rows

	cnt := 0
	for _, f := range rows {
		if f {
			cnt++
		}
	}
	return cnt
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

func (tg *transgrid) unset(c coord) {

	if c.y != tg.n {
		return
	}

	(*tg.rows)[c.x] = false
}

func (tg *transgrid) set(c coord) {

	if c.y != tg.n {
		return
	}

	_, has := (*tg.rows)[c.x]
	if has {
		return
	}

	if has && !(*tg.rows)[c.x] {
		return
	}

	(*tg.rows)[c.x] = true
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
