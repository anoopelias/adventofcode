package main

import (
	"fmt"
	goutils "go-utils"
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

func (c coord) copy() coord {
	return c
}

type bars struct {
	rbar []nxt
	dbar []nxt
	lbar []nxt
	ubar []nxt
}

type nxt struct {
	pos coord
	dir int
}

func (n *nxt) copy() nxt {
	return nxt{n.pos.copy(), n.dir}
}

type board struct {
	m    []string
	pos  *coord
	dir  int
	bars bars
	ubar []nxt
	dbar []nxt
	lbar []nxt
	rbar []nxt
}

func (b *board) move(num int) {
	for j := 0; j < num; j++ {
		switch b.dir {
		case left:
			b.moveL()
		case right:
			b.moveR()
		case up:
			b.moveU()
		case down:
			b.moveD()
		}
	}

}

func (b *board) moveR() {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.col = b.pos.col + 1
	if nxt.pos.col == len(b.m[b.pos.row]) || isB(b.m[b.pos.row][nxt.pos.col]) {
		nxt = b.bars.rbar[b.pos.row].copy()
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	}
}

func (b *board) moveL() {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.col = b.pos.col - 1
	if nxt.pos.col < 0 || isB(b.m[b.pos.row][nxt.pos.col]) {
		nxt = b.bars.lbar[b.pos.row].copy()
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	}
}

func (b *board) moveD() {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.row = b.pos.row + 1
	if nxt.pos.row == len(b.m) ||
		nxt.pos.col >= len(b.m[nxt.pos.row]) ||
		isB(b.m[nxt.pos.row][b.pos.col]) {

		nxt = b.bars.dbar[b.pos.col].copy()
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	}
}

func (b *board) moveU() {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.row = b.pos.row - 1
	if nxt.pos.row < 0 ||
		nxt.pos.col >= len(b.m[nxt.pos.row]) ||
		isB(b.m[nxt.pos.row][b.pos.col]) {

		nxt = b.bars.ubar[b.pos.col].copy()
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
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
		m:   ls,
		pos: &coord{},
		dir: right,
	}

	for j := 0; j < len(b.m[0]); j++ {
		if !isB(b.m[0][j]) {
			b.pos.col = j
			break
		}
	}
	return b
}

func solve(ls []string, typ int) string {
	b := newBoard(ls[:len(ls)-2])
	b.bars = calcBars(typ)
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

func calcBars(typ int) bars {

	if typ == 2 {
		// https: //imgur.com/a/VzxlUZa
		return bars{
			calcRBar(),
			calcDBar(),
			calcLBar(),
			calcUBar(),
		}
	}

	return bars{}
}

func calcLBar() []nxt {
	// input2: https://imgur.com/a/PmuuIlP
	lbar := make([]nxt, 200)

	// 0,50    149,0
	// 1,50    148,0
	for i := 0; i < 50; i++ {
		lbar[i] = nxt{coord{149 - i, 0}, right}
	}

	// 50,50    100,0
	// 51,50    100,1
	for i := 50; i < 100; i++ {
		lbar[i] = nxt{coord{100, i - 50}, down}
	}

	// 100,0    49,50
	// 101,0    48,50
	for i := 100; i < 150; i++ {
		lbar[i] = nxt{coord{149 - i, 50}, right}
	}

	// 150,0    0,50
	// 151,0    0,51
	for i := 150; i < 200; i++ {
		lbar[i] = nxt{coord{0, i - 100}, down}
	}
	return lbar
}

func calcRBar() []nxt {
	rbar := make([]nxt, 200)

	// 0,149    100,99
	// 1,149    101,99
	for i := 0; i < 50; i++ {
		rbar[i] = nxt{coord{i + 100, 99}, left}
	}

	// 50,99    49,100
	// 51,99    49,101
	for i := 50; i < 100; i++ {
		rbar[i] = nxt{coord{49, i + 50}, up}
	}

	// 100,99   0,149
	// 101,99   1,149
	for i := 100; i < 150; i++ {
		rbar[i] = nxt{coord{i - 100, 149}, left}
	}

	// 150,49    149,50
	// 151,49    149,51
	for i := 150; i < 200; i++ {
		rbar[i] = nxt{coord{149, i - 100}, up}
	}
	return rbar
}

func calcUBar() []nxt {
	ubar := make([]nxt, 150)

	// 100,0    50,50
	// 100,1    51,50
	for i := 0; i < 50; i++ {
		ubar[i] = nxt{coord{i + 50, 50}, right}
	}

	// 0,50     150,0
	// 0,51     151,0
	for i := 50; i < 100; i++ {
		ubar[i] = nxt{coord{i + 100, 0}, right}
	}

	// 0,100    199,0
	// 0,101    199,1
	for i := 100; i < 150; i++ {
		ubar[i] = nxt{coord{199, i - 100}, up}
	}

	return ubar
}

func calcDBar() []nxt {
	dbar := make([]nxt, 150)

	// 199,0    0,100
	// 199,1    0,101
	for i := 0; i < 50; i++ {
		dbar[i] = nxt{coord{0, i + 100}, down}

	}

	// 149,50    150,49
	// 149,51    151,49
	for i := 50; i < 100; i++ {
		dbar[i] = nxt{coord{i + 100, 49}, left}
	}

	// 49,100    99,50
	// 49,101    99,51
	for i := 100; i < 150; i++ {
		dbar[i] = nxt{coord{99, i - 50}, right}
	}

	return dbar
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
	name := "input2"
	typ := 2
	ls := goutils.LinesOf(name)
	fmt.Println(solve(ls, typ))
}
