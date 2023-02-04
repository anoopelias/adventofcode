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

type nxt struct {
	pos coord
	dir int
}

func (n *nxt) copy() nxt {
	return nxt{n.pos.copy(), n.dir}
}

type board struct {
	m   [][]string
	pos *coord
	dir int
	cb  cube
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
		nxt = b.cb.rnext(b.pos.row)
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
		nxt = b.cb.lnext(b.pos.row)
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

		nxt = b.cb.dnext(b.pos.col)
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

		nxt = b.cb.unext(b.pos.col)
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
	b.cb = getCube(typ)
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

func printMap(m [][]string) {
	for _, r := range m {
		fmt.Println(strings.Join(r, ""))
	}
}

func getCube(typ int) cube {
	if typ == 2 {
		return createCube()
	}

	return createOtherCube()
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
		top:  top * len,
		left: left * len,
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
	len int
}

func (cb *cube) rnext(row int) nxt {
	return cb.rbs[row/cb.len].next(row)
}
func (cb *cube) lnext(row int) nxt {
	return cb.lbs[row/cb.len].next(row)
}

func (cb *cube) unext(col int) nxt {
	return cb.ubs[col/cb.len].next(col)
}

func (cb *cube) dnext(col int) nxt {
	return cb.dbs[col/cb.len].next(col)
}

func createCube() cube {
	len := 50
	btf := newFace(0, 1, len)
	rtf := newFace(0, 2, len)
	ftf := newFace(1, 1, len)
	tpf := newFace(2, 1, len)
	ltf := newFace(2, 0, len)
	bkf := newFace(3, 0, len)

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
		len,
	}
}

func createOtherCube() cube {
	len := 4
	btf := newFace(1, 2, len)
	rtf := newFace(2, 3, len)
	ftf := newFace(2, 2, len)
	tpf := newFace(1, 0, len)
	ltf := newFace(1, 1, len)
	bkf := newFace(0, 2, len)

	bkf.ups.pairWith(tpf.ups, true)
	bkf.ls.pairWith(ltf.ups, false)
	bkf.rs.pairWith(rtf.rs, true)

	btf.rs.pairWith(rtf.ups, true)
	tpf.bts.pairWith(ftf.bts, true)
	tpf.ls.pairWith(rtf.bts, true)
	ftf.ls.pairWith(ltf.bts, true)

	return cube{
		&btf,
		&ftf,
		&rtf,
		&ltf,
		&tpf,
		&bkf,
		[]*side{tpf.ups, ltf.ups, bkf.ups, rtf.ups},
		[]*side{tpf.bts, ltf.bts, ftf.bts, rtf.bts},
		[]*side{bkf.ls, tpf.ls, ftf.ls},
		[]*side{bkf.rs, btf.rs, rtf.rs},
		len,
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
