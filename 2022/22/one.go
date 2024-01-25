package main

import (
	"fmt"
	goutils "go-utils"
	"math"
	"strconv"
	"strings"
)

const (
	blank int = iota
	open
	closed
)

const (
	// clockwise
	right int = iota
	down
	left
	up
)

type coord struct {
	row int
	col int
}

func (c *coord) setCol(col int) {
	c.col = col
}

type board struct {
	m    []string
	pos  *coord
	dir  int
	ubar []int
	dbar []int
	lbar []int
	rbar []int
}

func (b *board) move(num int) {
	switch b.dir {
	case left:
		b.moveL(num)
	case right:
		b.moveR(num)
	case up:
		b.moveU(num)
	case down:
		b.moveD(num)
	}

}

func (b *board) moveR(num int) {
	for j := 0; j < num; j++ {
		ncol := b.pos.col + 1
		if ncol == len(b.m[b.pos.row]) || isB(b.m[b.pos.row][ncol]) {
			ncol = b.lbar[b.pos.row]
		}
		if isO(b.m[b.pos.row][ncol]) {
			b.pos.col = ncol
		} else {
			break
		}
	}
}

func (b *board) moveL(num int) {
	for j := 0; j < num; j++ {
		ncol := b.pos.col - 1
		if ncol < 0 || isB(b.m[b.pos.row][ncol]) {
			ncol = b.rbar[b.pos.row]
		}
		if isO(b.m[b.pos.row][ncol]) {
			b.pos.col = ncol
		} else {
			break
		}
	}
}

func (b *board) moveD(num int) {
	for i := 0; i < num; i++ {
		nrow := b.pos.row + 1
		if nrow == len(b.m) ||
			b.pos.col >= len(b.m[nrow]) ||
			isB(b.m[nrow][b.pos.col]) {

			nrow = b.ubar[b.pos.col]
		}
		if isO(b.m[nrow][b.pos.col]) {
			b.pos.row = nrow
		} else {
			break
		}
	}
}

func (b *board) moveU(num int) {
	for i := 0; i < num; i++ {
		nrow := b.pos.row - 1
		if nrow < 0 ||
			b.pos.col >= len(b.m[nrow]) ||
			isB(b.m[nrow][b.pos.col]) {

			nrow = b.dbar[b.pos.col]
		}
		if isO(b.m[nrow][b.pos.col]) {
			b.pos.row = nrow
		} else {
			break
		}
	}
}

func (b *board) turnR() {
	if b.dir == up {
		b.dir = right
		return
	}
	b.dir++
}

func (b *board) turnL() {
	if b.dir == right {
		b.dir = up
		return
	}
	b.dir--
}

func newBoard(ls []string) board {
	b := board{
		ls,
		&coord{},
		right,
		make([]int, 0),
		make([]int, 0),
		make([]int, 0),
		make([]int, 0),
	}

	for i := 0; i < len(b.m); i++ {
		b.lbar = append(b.lbar, math.MaxInt)
		b.rbar = append(b.rbar, 0)
		for j := 0; j < len(b.m[i]); j++ {
			if len(b.ubar) < j+1 {
				b.ubar = append(b.ubar, math.MaxInt)
				b.dbar = append(b.dbar, 0)
			}
			if !isB(b.m[i][j]) {
				b.lbar[i] = goutils.Min(j, b.lbar[i])
				b.rbar[i] = goutils.Max(j, b.rbar[i])
				b.ubar[j] = goutils.Min(i, b.ubar[j])
				b.dbar[j] = goutils.Max(i, b.dbar[j])
			}
		}
	}

	for j := 0; j < len(b.m[0]); j++ {
		if !isB(b.m[0][j]) {
			b.pos.col = j
			break
		}
	}
	return b
}

func solve(ls []string) string {
	b := newBoard(ls[:len(ls)-2])
	ps := paths(ls[len(ls)-1])

	for _, p := range ps {
		switch p {
		case "L":
			b.turnL()
		case "R":
			b.turnR()
		default:
			num, _ := strconv.Atoi(p)
			b.move(num)
		}
	}

	fmt.Printf("%d %d %d\n", b.pos.row, b.pos.col, b.dir)
	return strconv.Itoa((1000 * (b.pos.row + 1)) + (4 * (b.pos.col + 1)) + b.dir)
}

func paths(path string) []string {
	path = strings.Join(strings.Split(path, "R"), ",R,")
	path = strings.Join(strings.Split(path, "L"), ",L,")

	return strings.Split(path, ",")
}

func isO(r byte) bool {
	return r == '.'
}

func isC(r byte) bool {
	return r == '#'
}

func isB(r byte) bool {
	return r == ' '
}

func main() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("input2")
	fmt.Println(solve(ls))
}
