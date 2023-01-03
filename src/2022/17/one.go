package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	pt := ls[0]
	pti := 0
	si := 0

	w := well{
		cont: make([][]int, 0),
	}

	for i := 0; i < 2022; i++ {
		sh := w.newShape(si)
		pti = settle(sh, pt, pti)
		//w.print()
		si = incrShape(si)
	}

	fmt.Println(w.tip() + 1)
}

func incrShape(i int) int {
	if i == SQ {
		i = HORIZ
	} else {
		i++
	}
	return i
}

func (w *well) newShape(i int) shape {
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

type shape interface {
	left() bool
	right() bool
	down() bool
}

func settle(s shape, p string, i int) int {
	for {
		if p[i] == '>' {
			s.right()
		} else {
			s.left()
		}
		i = incr(p, i)
		if !s.down() {
			break
		}
	}
	return i
}

type sq struct {
	w *well
	l int
	t int
}

func (w *well) newSq() *sq {
	w.addSpace(2)
	top := w.tip() + 5
	w.cont[top][2] = 2
	w.cont[top][3] = 2
	w.cont[top-1][2] = 2
	w.cont[top-1][3] = 2

	return &sq{
		w: w,
		t: top,
		l: 2,
	}
}

func (s *sq) left() bool {
	if s.l > 0 &&
		s.w.cont[s.t][s.l-1] == 0 &&
		s.w.cont[s.t-1][s.l-1] == 0 {

		s.w.cont[s.t][s.l+1] = 0
		s.w.cont[s.t-1][s.l+1] = 0

		s.w.cont[s.t][s.l-1] = 2
		s.w.cont[s.t-1][s.l-1] = 2

		s.l--
		return true
	}

	return false
}

func (s *sq) right() bool {
	if s.l < 5 &&
		s.w.cont[s.t][s.l+2] == 0 &&
		s.w.cont[s.t-1][s.l+2] == 0 {

		s.w.cont[s.t][s.l] = 0
		s.w.cont[s.t-1][s.l] = 0

		s.w.cont[s.t][s.l+2] = 2
		s.w.cont[s.t-1][s.l+2] = 2

		s.l++
		return true
	}

	return false
}

func (s *sq) down() bool {
	if s.t == 1 ||
		s.w.cont[s.t-2][s.l] == 1 ||
		s.w.cont[s.t-2][s.l+1] == 1 {

		s.w.cont[s.t][s.l] = 1
		s.w.cont[s.t][s.l+1] = 1
		s.w.cont[s.t-1][s.l] = 1
		s.w.cont[s.t-1][s.l+1] = 1
		return false
	}
	s.w.cont[s.t][s.l] = 0
	s.w.cont[s.t][s.l+1] = 0
	s.w.cont[s.t-2][s.l] = 2
	s.w.cont[s.t-2][s.l+1] = 2
	s.t--

	return true

}

type vert struct {
	w *well
	l int
	t int
}

func (w *well) newVert() *vert {
	w.addSpace(4)
	top := w.tip() + 7
	w.cont[top][2] = 2
	w.cont[top-1][2] = 2
	w.cont[top-2][2] = 2
	w.cont[top-3][2] = 2
	return &vert{
		w: w,
		t: top,
		l: 2,
	}
}

func (v *vert) left() bool {
	if v.l > 0 &&
		v.w.cont[v.t][v.l-1] == 0 &&
		v.w.cont[v.t-1][v.l-1] == 0 &&
		v.w.cont[v.t-2][v.l-1] == 0 &&
		v.w.cont[v.t-3][v.l-1] == 0 {

		v.w.cont[v.t][v.l] = 0
		v.w.cont[v.t-1][v.l] = 0
		v.w.cont[v.t-2][v.l] = 0
		v.w.cont[v.t-3][v.l] = 0

		v.w.cont[v.t][v.l-1] = 2
		v.w.cont[v.t-1][v.l-1] = 2
		v.w.cont[v.t-2][v.l-1] = 2
		v.w.cont[v.t-3][v.l-1] = 2

		v.l--

		return true
	}

	return false
}

func (v *vert) right() bool {

	if v.l < 6 &&
		v.w.cont[v.t][v.l+1] == 0 &&
		v.w.cont[v.t-1][v.l+1] == 0 &&
		v.w.cont[v.t-2][v.l+1] == 0 &&
		v.w.cont[v.t-3][v.l+1] == 0 {

		v.w.cont[v.t][v.l] = 0
		v.w.cont[v.t-1][v.l] = 0
		v.w.cont[v.t-2][v.l] = 0
		v.w.cont[v.t-3][v.l] = 0

		v.w.cont[v.t][v.l+1] = 2
		v.w.cont[v.t-1][v.l+1] = 2
		v.w.cont[v.t-2][v.l+1] = 2
		v.w.cont[v.t-3][v.l+1] = 2

		v.l++
		return true

	}

	return false
}

func (v *vert) down() bool {
	if v.t == 3 ||
		v.w.cont[v.t-4][v.l] == 1 {

		v.w.cont[v.t][v.l] = 1
		v.w.cont[v.t-1][v.l] = 1
		v.w.cont[v.t-2][v.l] = 1
		v.w.cont[v.t-3][v.l] = 1

		return false
	}

	v.w.cont[v.t][v.l] = 0
	v.w.cont[v.t-4][v.l] = 2

	v.t--
	return true
}

type ell struct {
	w *well
	l int
	t int
}

func (w *well) newEll() *ell {
	w.addSpace(3)
	top := w.tip() + 6
	w.cont[top][4] = 2
	w.cont[top-1][4] = 2
	w.cont[top-2][2] = 2
	w.cont[top-2][3] = 2
	w.cont[top-2][4] = 2

	return &ell{
		w: w,
		t: top,
		l: 2,
	}
}

func (e *ell) left() bool {
	if e.l > 0 &&
		e.w.cont[e.t][e.l+1] == 0 &&
		e.w.cont[e.t-1][e.l+1] == 0 &&
		e.w.cont[e.t-1][e.l-1] == 0 {

		e.w.cont[e.t][e.l+1] = 2
		e.w.cont[e.t][e.l+2] = 0

		e.w.cont[e.t-1][e.l+1] = 2
		e.w.cont[e.t-1][e.l+2] = 0

		e.w.cont[e.t-2][e.l-1] = 2
		e.w.cont[e.t-2][e.l+2] = 0

		e.l--
		return true
	}

	return false
}

func (e *ell) right() bool {
	if e.l < 4 &&
		e.w.cont[e.t][e.l+3] == 0 &&
		e.w.cont[e.t-1][e.l+3] == 0 &&
		e.w.cont[e.t-2][e.l+3] == 0 {

		e.w.cont[e.t][e.l+3] = 2
		e.w.cont[e.t][e.l+2] = 0

		e.w.cont[e.t-1][e.l+3] = 2
		e.w.cont[e.t-1][e.l+2] = 0

		e.w.cont[e.t-2][e.l+3] = 2
		e.w.cont[e.t-2][e.l] = 0
		e.l++

		return true

	}
	return false
}

func (e *ell) down() bool {

	if e.t == 2 ||
		e.w.cont[e.t-3][e.l] == 1 ||
		e.w.cont[e.t-3][e.l+1] == 1 ||
		e.w.cont[e.t-3][e.l+2] == 1 {

		e.w.cont[e.t][e.l+2] = 1
		e.w.cont[e.t-1][e.l+2] = 1
		e.w.cont[e.t-2][e.l] = 1
		e.w.cont[e.t-2][e.l+1] = 1
		e.w.cont[e.t-2][e.l+2] = 1

		return false
	}

	e.w.cont[e.t][e.l+2] = 0
	e.w.cont[e.t-2][e.l] = 0
	e.w.cont[e.t-2][e.l+1] = 0

	e.w.cont[e.t-3][e.l] = 2
	e.w.cont[e.t-3][e.l+1] = 2
	e.w.cont[e.t-3][e.l+2] = 2
	e.t--

	return true
}

type plus struct {
	w *well
	l int
	t int
}

func (w *well) newPlus() *plus {
	w.addSpace(3)
	top := w.tip() + 6
	w.cont[top][3] = 2
	w.cont[top-1][2] = 2
	w.cont[top-1][3] = 2
	w.cont[top-1][4] = 2
	w.cont[top-2][3] = 2

	return &plus{
		w: w,
		t: top,
		l: 2,
	}
}

func (p *plus) left() bool {
	if p.l > 0 &&
		p.w.cont[p.t][p.l] == 0 &&
		p.w.cont[p.t-1][p.l-1] == 0 &&
		p.w.cont[p.t-2][p.l] == 0 {

		p.w.cont[p.t][p.l] = 2
		p.w.cont[p.t][p.l+1] = 0

		p.w.cont[p.t-1][p.l-1] = 2
		p.w.cont[p.t-1][p.l+2] = 0

		p.w.cont[p.t-2][p.l] = 2
		p.w.cont[p.t-2][p.l+1] = 0

		p.l--

		return true

	}
	return false
}

func (p *plus) right() bool {
	if p.l < 4 &&
		p.w.cont[p.t][p.l+2] == 0 &&
		p.w.cont[p.t-1][p.l+3] == 0 &&
		p.w.cont[p.t-2][p.l+2] == 0 {

		p.w.cont[p.t][p.l+2] = 2
		p.w.cont[p.t][p.l+1] = 0

		p.w.cont[p.t-1][p.l+3] = 2
		p.w.cont[p.t-1][p.l] = 0

		p.w.cont[p.t-2][p.l+2] = 2
		p.w.cont[p.t-2][p.l+1] = 0

		p.l++

		return true

	}
	return false
}

func (p *plus) down() bool {
	if p.t == 2 ||
		p.w.cont[p.t-2][p.l] == 1 ||
		p.w.cont[p.t-2][p.l+2] == 1 ||
		p.w.cont[p.t-3][p.l+1] == 1 {

		p.w.cont[p.t][p.l+1] = 1
		p.w.cont[p.t-1][p.l] = 1
		p.w.cont[p.t-1][p.l+1] = 1
		p.w.cont[p.t-1][p.l+2] = 1
		p.w.cont[p.t-2][p.l+1] = 1
		return false
	}

	p.w.cont[p.t][p.l+1] = 0
	p.w.cont[p.t-1][p.l] = 0
	p.w.cont[p.t-1][p.l+2] = 0
	p.w.cont[p.t-2][p.l] = 2
	p.w.cont[p.t-2][p.l+2] = 2
	p.w.cont[p.t-3][p.l+1] = 2

	p.t--
	return true

}

type horiz struct {
	w *well
	l int
	t int
}

func (h *horiz) well() *well {
	return h.w
}

func (h *horiz) right() bool {
	if h.l+4 < 7 && h.w.cont[h.t][h.l+4] == 0 {
		h.w.cont[h.t][h.l] = 0
		h.w.cont[h.t][h.l+4] = 2
		h.l += 1
		return true
	}
	return false
}

func (h *horiz) left() bool {
	if h.l > 0 && h.w.cont[h.t][h.l-1] == 0 {
		h.w.cont[h.t][h.l+3] = 0
		h.w.cont[h.t][h.l-1] = 2
		h.l -= 1
		return true
	}
	return false
}
func (h *horiz) down() bool {
	if h.t == 0 ||
		h.w.cont[h.t-1][h.l] == 1 ||
		h.w.cont[h.t-1][h.l+1] == 1 ||
		h.w.cont[h.t-1][h.l+2] == 1 ||
		h.w.cont[h.t-1][h.l+3] == 1 {

		h.w.cont[h.t][h.l] = 1
		h.w.cont[h.t][h.l+1] = 1
		h.w.cont[h.t][h.l+2] = 1
		h.w.cont[h.t][h.l+3] = 1

		return false
	}
	h.w.cont[h.t][h.l] = 0
	h.w.cont[h.t][h.l+1] = 0
	h.w.cont[h.t][h.l+2] = 0
	h.w.cont[h.t][h.l+3] = 0

	h.w.cont[h.t-1][h.l] = 2
	h.w.cont[h.t-1][h.l+1] = 2
	h.w.cont[h.t-1][h.l+2] = 2
	h.w.cont[h.t-1][h.l+3] = 2
	h.t -= 1

	return true
}

func (w *well) newHoriz() *horiz {
	w.addSpace(1)
	top := w.tip() + 4
	w.cont[top][2] = 2
	w.cont[top][3] = 2
	w.cont[top][4] = 2
	w.cont[top][5] = 2

	return &horiz{
		w: w,
		t: top,
		l: 2,
	}
}

func (w *well) print() {
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
