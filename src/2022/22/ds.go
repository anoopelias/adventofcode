package main

const len = 50

type coord struct {
	row int
	col int
}

type side struct {
	from    int
	to      int
	c       int
	horiz   bool
	pair    *side
	pairinv bool
}

func (s *side) pairWith(ps *side, inv bool) {
	s.pair = ps
	s.pairinv = inv
	ps.pair = s
	ps.pairinv = inv
}

func (s *side) toCoord(l int) coord {
	if s.horiz {
		return coord{s.c, s.from + l}
	}
	return coord{s.from + l, s.c}
}

func (s *side) toInvCoord(l int) coord {
	if s.horiz {
		return coord{s.c, s.to - l - 1}
	}
	return coord{s.to - l - 1, s.c}
}

func (s *side) next(l int) coord {
	if !s.pairinv {
		return s.pair.toCoord(l)
	}

	return s.pair.toInvCoord(l)
}

type face struct {
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
		to:    f.top + len,
		c:     f.left,
		horiz: false,
	}
	f.rs = &side{
		from:  f.top,
		to:    f.top + len,
		c:     f.left + len - 1,
		horiz: false,
	}
	f.ups = &side{
		from:  f.left,
		to:    f.left + len,
		c:     f.top,
		horiz: true,
	}
	f.bts = &side{
		from:  f.left,
		to:    f.left + len,
		c:     f.top + len - 1,
		horiz: true,
	}
}

func newFace(top, left int) face {
	f := face{
		top:  0,
		left: 50,
	}
	f.fillSides()
	return f
}

func main() {
	btf := newFace(0, 50)
	rtf := newFace(0, 100)
	ftf := newFace(50, 50)
	tpf := newFace(100, 50)
	ltf := newFace(100, 0)
	bkf := newFace(150, 0)

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
}
