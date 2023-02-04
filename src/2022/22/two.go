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
	m    [][]string
	pos  *coord
	dir  int
	bars bars
	ubar []nxt
	dbar []nxt
	lbar []nxt
	rbar []nxt
}

func (b *board) move(num int) {
	bkd := false
	for j := 0; j < num && !bkd; j++ {
		switch b.dir {
		case left:
			bkd = b.moveL()
		case right:
			bkd = b.moveR()
		case up:
			bkd = b.moveU()
		case down:
			bkd = b.moveD()
		}
		b.m[b.pos.row][b.pos.col] = dirStr(b.dir)
	}

}

func dirStr(dir int) string {
	switch dir {
	case right:
		return ">"
	case down:
		return "v"
	case left:
		return "<"
	case up:
		return "^"
	}
	return "."
}

func (b *board) moveR() bool {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.col = b.pos.col + 1
	if nxt.pos.col == len(b.m[b.pos.row]) || isB(b.m[b.pos.row][nxt.pos.col]) {
		nxt = b.bars.rbar[b.pos.row].copy()
		// fmt.Printf("> %d,%d -> %d,%d %s\n", b.pos.row, b.pos.col, nxt.pos.row, nxt.pos.col, dirStr(nxt.dir))
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	} else {
		return true
	}
	return false
}

func (b *board) moveL() bool {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.col = b.pos.col - 1
	if nxt.pos.col < 0 || isB(b.m[b.pos.row][nxt.pos.col]) {
		nxt = b.bars.lbar[b.pos.row].copy()
		// fmt.Printf("< %d,%d -> %d,%d %s\n", b.pos.row, b.pos.col, nxt.pos.row, nxt.pos.col, dirStr(nxt.dir))
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	} else {
		return true
	}
	return false
}

func (b *board) moveD() bool {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.row = b.pos.row + 1
	if nxt.pos.row == len(b.m) ||
		nxt.pos.col >= len(b.m[nxt.pos.row]) ||
		isB(b.m[nxt.pos.row][b.pos.col]) {

		nxt = b.bars.dbar[b.pos.col].copy()
		// fmt.Printf("v %d,%d -> %d,%d %s\n", b.pos.row, b.pos.col, nxt.pos.row, nxt.pos.col, dirStr(nxt.dir))
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	} else {
		return true
	}
	return false
}

func (b *board) moveU() bool {
	nxt := nxt{b.pos.copy(), b.dir}
	nxt.pos.row = b.pos.row - 1
	if nxt.pos.row < 0 ||
		nxt.pos.col >= len(b.m[nxt.pos.row]) ||
		isB(b.m[nxt.pos.row][b.pos.col]) {

		nxt = b.bars.ubar[b.pos.col].copy()
		// fmt.Printf("^ %d,%d -> %d,%d %s\n", b.pos.row, b.pos.col, nxt.pos.row, nxt.pos.col, dirStr(nxt.dir))
	}
	if isO(b.m[nxt.pos.row][nxt.pos.col]) {
		b.pos = &nxt.pos
		b.dir = nxt.dir
	} else {
		return true
	}
	return false
}

func (b *board) turnR() {
	if b.dir == up {
		b.dir = right
		return
	}
	b.dir++
	b.m[b.pos.row][b.pos.col] = dirStr(b.dir)
}

func (b *board) turnL() {
	if b.dir == right {
		b.dir = up
		return
	}
	b.dir--
	b.m[b.pos.row][b.pos.col] = dirStr(b.dir)
}

func newBoard(ls []string) board {
	m := make([][]string, 0)

	for _, l := range ls {
		m = append(m, strings.Split(l, ""))
	}

	b := board{
		m:   m,
		pos: &coord{},
		dir: right,
	}

	for j := 0; j < len(b.m[0]); j++ {
		if !isB(b.m[0][j]) {
			b.pos.col = j
			break
		}
	}
	b.m[b.pos.row][b.pos.col] = ">"
	return b
}

func solve(ls []string, typ int) string {
	b := newBoard(ls[:len(ls)-2])
	b.bars = calcBars(typ)
	ps := paths(ls[len(ls)-1])

	// printMap(b.m)
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

		// if i == 3 {
		// 	fmt.Printf("%s\n", p)
		// 	printMap(b.m)
		// }
	}
	// fmt.Println()
	// printMap(b.m)

	fmt.Printf("%d %d %d\n", b.pos.row, b.pos.col, b.dir)
	return strconv.Itoa((1000 * (b.pos.row + 1)) + (4 * (b.pos.col + 1)) + b.dir)
}

func printMap(m [][]string) {
	for _, r := range m {
		fmt.Println(strings.Join(r, ""))
	}
}

func calcBars(typ int) bars {

	if typ == 2 {
		// https://imgur.com/a/VzxlUZa
		cube := createCube()
		return bars{
			calcRBar(cube),
			calcDBar(cube),
			calcLBar(cube),
			calcUBar(cube),
		}
	}

	return bars{
		calcOtherRBar(),
		calcOtherDBar(),
		calcOtherLBar(),
		calcOtherUBar(),
	}
}

func calcOtherLBar() []nxt {
	lbar := make([]nxt, 12)

	// 0,8   4,4
	// 1,8   4,5
	for i := 0; i < 4; i++ {
		lbar[i] = nxt{coord{4, i + 4}, down}
	}

	// 4,0    11,15
	// 5,0    11,14
	for i := 4; i < 8; i++ {
		lbar[i] = nxt{coord{11, 19 - i}, up}
	}

	// 8,0    7,7
	// 9,0    7,6
	for i := 8; i < 12; i++ {
		lbar[i] = nxt{coord{7, 15 - i}, up}
	}

	return lbar
}

func calcOtherRBar() []nxt {
	rbar := make([]nxt, 12)

	// 0,11   11,15
	// 1,11   10,15
	for i := 0; i < 4; i++ {
		rbar[i] = nxt{coord{11 - i, 15}, left}
	}

	// 4,11    8,15
	// 5,11    8,14
	for i := 4; i < 8; i++ {
		rbar[i] = nxt{coord{8, 19 - i}, down}
	}

	// 8,15    3,11
	// 9,15    2,11
	for i := 8; i < 12; i++ {
		rbar[i] = nxt{coord{11 - i, 11}, left}
	}

	return rbar
}

func calcOtherUBar() []nxt {
	ubar := make([]nxt, 16)

	// 3,0   0,11
	// 3,1   0,10
	for i := 0; i < 4; i++ {
		ubar[i] = nxt{coord{0, 11 - i}, down}
	}

	// 3,4    0,8
	// 3,5    1,8
	for i := 4; i < 8; i++ {
		ubar[i] = nxt{coord{i - 4, 8}, right}
	}

	// 0,8    4,3
	// 0,9    4,2
	for i := 8; i < 12; i++ {
		ubar[i] = nxt{coord{4, 11 - i}, down}
	}

	// 8,12    7,11
	// 8,13    6,11
	for i := 12; i < 16; i++ {
		ubar[i] = nxt{coord{19 - i, 11}, left}
	}

	return ubar
}

func calcOtherDBar() []nxt {
	dbar := make([]nxt, 16)

	// dbar[i] = nxt{coord{}, 0}

	// 7,0    11,11
	// 7,1    11,10
	for i := 0; i < 4; i++ {
		dbar[i] = nxt{coord{11, 11 - i}, up}
	}

	// 7,4    11,8
	// 7,5    10,8
	for i := 4; i < 8; i++ {
		dbar[i] = nxt{coord{15 - i, 8}, right}
	}

	// 11,8    7,3
	// 11,9    7,2
	for i := 8; i < 12; i++ {
		dbar[i] = nxt{coord{7, 11 - i}, up}
	}

	// 11,12    7,0
	// 11,13    6,0
	for i := 12; i < 16; i++ {
		dbar[i] = nxt{coord{19 - i, 0}, up}
	}
	return dbar
}

func calcLBar(cb cube) []nxt {
	lbar := make([]nxt, 200)

	for i := 0; i < 200; i++ {
		lbar[i] = cb.lbs[i/50].next(i)
	}
	return lbar
}

func calcRBar(cb cube) []nxt {
	rbar := make([]nxt, 200)

	for i := 0; i < 200; i++ {
		rbar[i] = cb.rbs[i/50].next(i)
	}
	return rbar
}

func calcUBar(cb cube) []nxt {
	ubar := make([]nxt, 150)
	for i := 0; i < 150; i++ {
		ubar[i] = cb.ubs[i/50].next(i)
	}
	return ubar
}

func calcDBar(cb cube) []nxt {
	dbar := make([]nxt, 150)
	for i := 0; i < 150; i++ {
		dbar[i] = cb.dbs[i/50].next(i)

	}
	return dbar
}

func paths(path string) []string {
	path = strings.Join(strings.Split(path, "R"), ",R,")
	path = strings.Join(strings.Split(path, "L"), ",L,")

	return strings.Split(path, ",")
}

func isO(r string) bool {
	return r == "." || r == ">" || r == "<" || r == "v" || r == "^"
}

func isC(r byte) bool {
	return r == '#'
}

func isB(r string) bool {
	return r == " "
}

type side struct {
	from    int
	to      int
	c       int
	horiz   bool
	first   bool
	pair    *side
	pairinv bool
}

func (s *side) pairWith(ps *side, inv bool) {
	s.pair = ps
	s.pairinv = inv
	ps.pair = s
	ps.pairinv = inv
}

func (s *side) goInDir() int {
	if s.horiz {
		if s.first {
			return down
		}
		return up
	}
	if s.first {
		return right
	}
	return left
}

func (s *side) toCoord(l int) nxt {
	if s.horiz {
		return nxt{coord{s.c, s.from + l}, s.goInDir()}
	}
	return nxt{coord{s.from + l, s.c}, s.goInDir()}
}

func (s *side) toInvCoord(l int) nxt {
	if s.horiz {
		return nxt{coord{s.c, s.to - l - 1}, s.goInDir()}
	}
	return nxt{coord{s.to - l - 1, s.c}, s.goInDir()}
}

func (s *side) next(l int) nxt {
	if !s.pairinv {
		return s.pair.toCoord(l - s.from)
	}

	return s.pair.toInvCoord(l - s.from)
}

type face struct {
	len  int
	top  int
	left int
	ups  *side
	bts  *side
	ls   *side
	rs   *side
}

func (f *face) fillSides() {
	f.ls = &side{
		from:  f.top,
		to:    f.top + f.len,
		c:     f.left,
		horiz: false,
		first: true,
	}
	f.rs = &side{
		from:  f.top,
		to:    f.top + f.len,
		c:     f.left + f.len - 1,
		horiz: false,
		first: false,
	}
	f.ups = &side{
		from:  f.left,
		to:    f.left + f.len,
		c:     f.top,
		horiz: true,
		first: true,
	}
	f.bts = &side{
		from:  f.left,
		to:    f.left + f.len,
		c:     f.top + f.len - 1,
		horiz: true,
		first: false,
	}
}

func newFace(top, left, len int) face {
	f := face{
		len:  len,
		top:  top,
		left: left,
	}
	f.fillSides()
	return f
}

type cube struct {
	btf *face
	ftf *face
	rtf *face
	ltf *face
	tpf *face
	bkf *face

	ubs []*side
	dbs []*side
	lbs []*side
	rbs []*side
}

func createCube() cube {
	len := 50
	btf := newFace(0, 50, len)
	rtf := newFace(0, 100, len)
	ftf := newFace(50, 50, len)
	tpf := newFace(100, 50, len)
	ltf := newFace(100, 0, len)
	bkf := newFace(150, 0, len)

	btf.ls.pairWith(ltf.ls, true)
	btf.ups.pairWith(bkf.ls, false)
	btf.rs.pairWith(rtf.ls, false)
	btf.bts.pairWith(ftf.ups, false)

	rtf.ups.pairWith(bkf.bts, false)
	rtf.rs.pairWith(tpf.rs, true)
	rtf.bts.pairWith(ftf.rs, false)

	ftf.bts.pairWith(tpf.ups, false)
	ftf.ls.pairWith(ltf.ups, false)

	tpf.ls.pairWith(ltf.rs, false)
	tpf.bts.pairWith(bkf.rs, false)

	ltf.bts.pairWith(bkf.ups, false)

	return cube{
		&btf,
		&ftf,
		&rtf,
		&ltf,
		&tpf,
		&bkf,
		[]*side{ltf.ups, btf.ups, rtf.ups},
		[]*side{bkf.bts, tpf.bts, rtf.bts},
		[]*side{btf.ls, ftf.ls, ltf.ls, bkf.ls},
		[]*side{rtf.rs, ftf.rs, tpf.rs, bkf.rs},
	}
}

func main() {
	fmt.Println("Starting...")

	name := "input"
	typ := 1
	ls := goutils.LinesOf(name)
	fmt.Println(solve(ls, typ))

	name = "input2"
	typ = 2
	ls = goutils.LinesOf(name)
	fmt.Println(solve(ls, typ))
}
