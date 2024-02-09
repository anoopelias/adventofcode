package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type coord struct {
	x int
	y int
}

type xline struct {
	minx int
	maxx int
}

type senbeac struct {
	s coord
	b coord
}

type transgrid struct {
	minx int
	miny int
	n    int
	rows *[]*[]*xline
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/15/input2")

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

	tg := newGrid(minx, miny, maxx, maxy, 4000000)
	//tg := newGrid(minx, miny, maxx, maxy, 20)

	for _, sb := range sbs {
		tg.setRow(sb.s.y, sb.s.x, 0)
		tg.setRow(sb.b.y, sb.b.x, 0)
		md := mandist(sb.s, sb.b)
		tg.setScanner(sb.s, md)
	}

	for i := 0; i < tg.n; i++ {
		f, x, y := tg.checkRow(i)
		if !f {
			fmt.Printf("%d, %d\n", x, y)
			fmt.Print(x*4000000 + y)
			break
		}
	}

}

func newGrid(minx, miny, maxx, maxy int, n int) transgrid {
	rows := make([]*[]*xline, n+1)
	fmt.Println(len(rows))
	return transgrid{
		minx: minx,
		miny: miny,
		n:    n,
		rows: &rows,
	}
}

func (tg *transgrid) checkRow(r int) (bool, int, int) {
	row := *(*tg.rows)[r]
	sort.SliceStable(row, func(i, j int) bool {
		if (row)[i].minx < (row)[j].minx {
			return true
		} else if (row)[i].minx > (row)[j].minx {
			return false
		} else if (row)[i].maxx < (row)[j].maxx {
			return true
		} else if (row)[i].maxx > (row)[j].maxx {
			return false
		}
		return false
	})

	n := 0
	for i := 0; i < len(row); i++ {
		if row[i].minx > n {
			return false, n, r
		}
		if (row[i].maxx + 1) > n {
			n = row[i].maxx + 1
		}
	}

	if n <= tg.n {
		return false, n, r
	}

	return true, -1, -1
}

func (tg *transgrid) setScanner(s coord, md int) {
	for i := s.y - md; i <= s.y+md; i++ {
		tg.setRow(i, s.x, md-abs(s.y-i))
	}

}

func (tg *transgrid) setRow(r int, c int, xdist int) {
	if r < 0 || r > tg.n {
		return
	}
	if (*tg.rows)[r] == nil {
		nr := make([]*xline, 0)
		(*tg.rows)[r] = &nr
	}

	(*(*tg.rows)[r]) = append((*(*tg.rows)[r]), &xline{c - xdist, c + xdist})
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
