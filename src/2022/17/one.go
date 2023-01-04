package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

const FLAG = 0
const LOOPS = 2022
const PRINT = 0
const NO_OF_SHAPES = 5

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	pt := ls[0]
	pti := 0

	w := well{
		cont: make([][]int, 0),
	}

	for i := 0; i < LOOPS; i++ {
		sh := w.newShape(i % NO_OF_SHAPES)
		w.print()
		pti = settle(sh, pt, pti)
	}

	w.print()

	fmt.Println(w.tip() + 1)
}

func (w *well) newShape(i int) ishape {
	switch i {
	case HORIZ:
		return w.newHoriz()
	case PLUS:
		return w.newPlus()
	case ELL:
		return w.newEll()
	case VERT:
		return w.newVert()
	case SQ:
		return w.newSq()

	}
	return nil
}

const (
	HORIZ int = iota
	PLUS
	ELL
	VERT
	SQ
)

type well struct {
	cont [][]int
}

type ishape interface {
	left() bool
	right() bool
	down() bool
	print()
}

func settle(s ishape, p string, i int) int {
	for {
		//fmt.Printf("i %d, %c\n", i, p[i])
		if p[i] == '>' {
			s.right()
		} else {
			s.left()
		}
		s.print()
		i = incr(p, i)
		if !s.down() {
			s.print()
			break
		}
	}
	return i
}

func (w *well) newSq() ishape {
	return newShape(w, props{
		h: 2,
		w: 2,
		fill: []pos{
			{0, 0},
			{0, 1},
			{-1, 0},
			{-1, 1},
		},

		lft: []pos{
			{0, -1},
			{-1, -1},
		},
		lfth: []pos{
			{0, 1},
			{-1, 1},
		},

		rt: []pos{
			{0, 2},
			{-1, 2},
		},
		rth: []pos{
			{0, 0},
			{-1, 0},
		},

		dwn: []pos{
			{-2, 0},
			{-2, 1},
		},
		dwnh: []pos{
			{0, 0},
			{0, 1},
		},
	})
}

func (w *well) newVert() ishape {
	return newShape(w, props{
		h: 4,
		w: 1,
		fill: []pos{
			{0, 0},
			{-1, 0},
			{-2, 0},
			{-3, 0},
		},

		lft: []pos{
			{0, -1},
			{-1, -1},
			{-2, -1},
			{-3, -1},
		},
		lfth: []pos{
			{0, 0},
			{-1, 0},
			{-2, 0},
			{-3, 0},
		},

		rt: []pos{
			{0, 1},
			{-1, 1},
			{-2, 1},
			{-3, 1},
		},
		rth: []pos{
			{0, 0},
			{-1, 0},
			{-2, 0},
			{-3, 0},
		},

		dwn: []pos{
			{-4, 0},
		},
		dwnh: []pos{
			{0, 0},
		},
	})
}

func (w *well) newEll() ishape {
	return newShape(w, props{
		h: 3,
		w: 3,
		fill: []pos{
			{0, 2},
			{-1, 2},
			{-2, 0},
			{-2, 1},
			{-2, 2},
		},

		lft: []pos{
			{0, 1},
			{-1, 1},
			{-2, -1},
		},
		lfth: []pos{
			{0, 2},
			{-1, 2},
			{-2, 2},
		},

		rt: []pos{
			{0, 3},
			{-1, 3},
			{-2, 3},
		},
		rth: []pos{
			{0, 2},
			{-1, 2},
			{-2, 0},
		},

		dwn: []pos{
			{-3, 0},
			{-3, 1},
			{-3, 2},
		},
		dwnh: []pos{
			{0, 2},
			{-2, 0},
			{-2, 1},
		},
	})
}

func (w *well) newPlus() ishape {
	return newShape(w, props{
		h: 3,
		w: 3,
		fill: []pos{
			{0, 1},
			{-1, 0},
			{-1, 1},
			{-1, 2},
			{-2, 1},
		},

		lft: []pos{
			{0, 0},
			{-1, -1},
			{-2, 0},
		},
		lfth: []pos{
			{0, 1},
			{-1, 2},
			{-2, 1},
		},

		rt: []pos{
			{0, 2},
			{-1, 3},
			{-2, 2},
		},
		rth: []pos{
			{0, 1},
			{-1, 0},
			{-2, 1},
		},

		dwn: []pos{
			{-2, 0},
			{-2, 2},
			{-3, 1},
		},
		dwnh: []pos{
			{0, 1},
			{-1, 0},
			{-1, 2},
		},
	})
}

func (w *well) newHoriz() ishape {
	return newShape(w, props{
		h: 1,
		w: 4,
		fill: []pos{
			{0, 0},
			{0, 1},
			{0, 2},
			{0, 3},
		},

		lft: []pos{
			{0, -1},
		},
		lfth: []pos{
			{0, 3},
		},

		rt: []pos{
			{0, 4},
		},
		rth: []pos{
			{0, 0},
		},

		dwn: []pos{
			{-1, 0},
			{-1, 1},
			{-1, 2},
			{-1, 3},
		},
		dwnh: []pos{
			{0, 0},
			{0, 1},
			{0, 2},
			{0, 3},
		},
	})
}

func (w *well) print() {
	if PRINT == 1 {
		for i := len(w.cont) - 1; i >= 0; i-- {
			fmt.Print("|")
			for _, c := range w.cont[i] {
				fmt.Print(toChar(c) + " ")
			}
			fmt.Print("|")
			fmt.Println()
		}
		fmt.Println("+-------+")
	}
}

func (w *well) addSpace(top int) {
	for len(w.cont)-w.tip() <= 3+top {
		w.addEmptyRow()
	}
}

func (w *well) tip() int {
	for i := len(w.cont) - 1; i >= 0; i-- {
		for _, c := range w.cont[i] {
			if c == 1 {
				return i
			}
		}
	}
	return -1
}

func (w *well) addEmptyRow() {
	w.cont = append(w.cont, []int{0, 0, 0, 0, 0, 0, 0})
}

func incr(p string, i int) int {
	i++
	if i == len(p) {
		i = 0
	}
	return i
}

func toChar(c int) string {
	if c == 0 {
		return "."
	}
	if c == 1 {
		return "#"
	}
	return "@"
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

type shape struct {
	p props
	w *well
	t int
	l int
}

type props struct {
	h    int
	w    int
	fill []pos

	lft  []pos
	lfth []pos

	rt  []pos
	rth []pos

	dwn  []pos
	dwnh []pos
}

const (
	FREE int = iota
	FILLED
	MOVING
)

type pos struct {
	p int
	q int
}

func newShape(w *well, p props) ishape {
	w.addSpace(p.h)
	t := w.tip() + 3 + p.h
	l := 2

	for _, pos := range p.fill {
		w.cont[t+pos.p][l+pos.q] = 2
	}

	return &shape{
		w: w,
		t: t,
		l: l,
		p: p,
	}
}

func (sh *shape) print() {
	sh.w.print()
}

func (sh *shape) test(ps []pos) bool {
	for _, pos := range ps {
		if sh.w.cont[sh.t+pos.p][sh.l+pos.q] == 1 {
			return false
		}
	}
	return true
}

func (sh *shape) fill(ps []pos, ty int) {
	for _, pos := range ps {
		sh.w.cont[sh.t+pos.p][sh.l+pos.q] = ty
	}
}

func (sh *shape) left() bool {
	if sh.l == 0 {
		return false
	}
	if !sh.test(sh.p.lft) {
		return false
	}

	sh.fill(sh.p.lft, MOVING)
	sh.fill(sh.p.lfth, FREE)

	sh.l--
	return true
}

func (sh *shape) right() bool {
	if sh.l+sh.p.w == 7 {
		return false
	}
	if !sh.test(sh.p.rt) {
		return false
	}

	sh.fill(sh.p.rt, MOVING)
	sh.fill(sh.p.rth, FREE)

	sh.l++
	return true
}

func (sh *shape) down() bool {
	if sh.t == sh.p.h-1 || !sh.test(sh.p.dwn) {
		sh.fill(sh.p.fill, FILLED)
		return false
	}

	sh.fill(sh.p.dwn, MOVING)
	sh.fill(sh.p.dwnh, FREE)

	sh.t--
	return true
}
