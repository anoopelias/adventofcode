package main

import (
	"fmt"
	goutils "go-utils"
	"strconv"
)

const (
	north int = iota
	south
	east
	west
)

type coord struct {
	top  int
	left int
}

func (c *coord) hash() string {
	return strconv.Itoa(c.top) + ":" + strconv.Itoa(c.left)
}

func (c *coord) north(h int) coord {
	if c.top == 1 {
		return coord{
			h - 2,
			c.left,
		}
	}
	return coord{
		c.top - 1,
		c.left,
	}
}

func (c *coord) south(h int) coord {
	if c.top == h-2 {
		return coord{
			1,
			c.left,
		}
	}
	return coord{
		c.top + 1,
		c.left,
	}
}

func (c *coord) west(w int) coord {
	if c.left == 1 {
		return coord{
			c.top,
			w - 2,
		}
	}
	return coord{
		c.top,
		c.left - 1,
	}
}

func (c *coord) east(w int) coord {
	if c.left == w-2 {
		return coord{
			c.top,
			1,
		}
	}
	return coord{
		c.top,
		c.left + 1,
	}
}

func (c *coord) isOpen(h, w int) bool {
	if c.top == 0 && c.left == 1 {
		return true
	}

	if c.top == h-1 && c.left == w-2 {
		return true
	}

	if c.top == 0 || c.left == 0 || c.top == h-1 || c.left == w-1 {
		return false
	}

	return true
}

type bliz struct {
	v   *valley
	dir int
}

func newBliz(c rune, v *valley) *bliz {
	switch c {
	case '^':
		return &bliz{v, north}
	case '>':
		return &bliz{v, east}
	case 'v':
		return &bliz{v, south}
	case '<':
		return &bliz{v, west}
	default:
		return nil
	}
}

func (b *bliz) dirString() string {
	switch b.dir {
	case north:
		return "^"
	case south:
		return "v"
	case east:
		return ">"
	case west:
		return "<"
	default:
		return ""
	}
}

func (b *bliz) next(c coord, h int, w int) coord {
	switch b.dir {
	case north:
		return c.north(h)
	case south:
		return c.south(h)
	case west:
		return c.west(w)
	case east:
		return c.east(w)
	}

	return coord{}
}

type valley struct {
	w      int
	h      int
	rep    int
	exp    coord
	free   []map[string]bool
	state  []string
	nbrmap map[string][]coord
	marked map[string]bool
}

func newValley(ls []string) valley {

	v := valley{
		exp:    coord{0, 1},
		nbrmap: make(map[string][]coord),
		marked: make(map[string]bool),
	}
	vmap := make([][][]*bliz, 0)
	for _, l := range ls {
		r := make([][]*bliz, 0)
		for _, c := range l {
			bz := newBliz(c, &v)
			if bz != nil {
				r = append(r, []*bliz{bz})
			} else {
				r = append(r, []*bliz{})
			}
		}
		vmap = append(vmap, r)
	}

	v.h, v.w = len(vmap), len(vmap[0])
	v.rep = lcm(v.h-2, v.w-2)
	v.free = make([]map[string]bool, v.rep)
	v.state = make([]string, v.rep)
	for i := 0; i < v.rep; i++ {
		v.free[i], v.state[i] = getFree(vmap)
		vmap = nextMin(vmap)
	}

	return v
}

func printVmap(vmap [][][]*bliz) {
	h, w := len(vmap), len(vmap[0])
	fmt.Println()
	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			if len(vmap[i][j]) == 0 {
				fmt.Print(".")
			} else if len(vmap[i][j]) == 1 {
				fmt.Print(vmap[i][j][0].dirString())
			} else {
				fmt.Print(len(vmap[i][j]))
			}
		}
		fmt.Println()
	}

}

func nextMin(vmap [][][]*bliz) [][][]*bliz {
	nvmap := make([][][]*bliz, 0)
	h, w := len(vmap), len(vmap[0])

	for i := 0; i < h; i++ {
		nr := make([][]*bliz, 0)
		for j := 0; j < w; j++ {
			nr = append(nr, []*bliz{})
		}
		nvmap = append(nvmap, nr)
	}

	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			for _, b := range vmap[i][j] {
				c := coord{i, j}
				nc := b.next(c, h, w)
				nvmap[nc.top][nc.left] = append(nvmap[nc.top][nc.left], b)
			}
		}
	}
	return nvmap
}

func getFree(vmap [][][]*bliz) (map[string]bool, string) {

	fr := make(map[string]bool)
	h, w := len(vmap), len(vmap[0])
	state := "::"
	for i := 1; i < h-1; i++ {
		for j := 1; j < w-1; j++ {
			c := coord{i, j}
			if len(vmap[i][j]) == 0 {
				fr[c.hash()] = true
				state += c.hash() + "::"
			}
		}
	}

	// Assuming source and target location will be free at all times.
	fr[(&coord{h - 1, w - 2}).hash()] = true
	fr[(&coord{0, 1}).hash()] = true
	return fr, state
}

func (v *valley) neighbors(min int, c coord) []coord {

	min = min % v.rep
	key := strconv.Itoa(min) + "::" + c.hash()

	if cs, ok := v.nbrmap[key]; ok {
		return cs
	}

	cs := make([]coord, 0)
	ps := []coord{
		c,
		{c.top - 1, c.left},
		{c.top + 1, c.left},
		{c.top, c.left - 1},
		{c.top, c.left + 1}}

	for _, p := range ps {
		if p.isOpen(v.h, v.w) {
			if _, ok := v.free[min][p.hash()]; ok {
				cs = append(cs, p)
			}

		}
	}
	v.nbrmap[key] = cs
	return cs
}

func (v *valley) hashOf(min int) string {
	return v.state[min%v.rep]
}

type bfsNode struct {
	min int
	pos coord
}

func (n *bfsNode) hash(v *valley) string {
	return v.hashOf(n.min) + "_" + n.pos.hash()
}

func (v *valley) trip(min int, from coord, to coord) int {
	q := []bfsNode{{min, from}}
	v.marked = make(map[string]bool)
	var hd bfsNode

	for len(q) > 0 {
		hd, q = q[0], q[1:]
		ns := v.neighbors(hd.min+1, hd.pos)

		hash := hd.hash(v)
		if _, ok := v.marked[hash]; ok {
			continue
		}
		v.marked[hash] = true

		if hd.pos == to {
			min = hd.min
			break
		}

		for _, n := range ns {
			q = append(q, bfsNode{hd.min + 1, n})
		}
	}

	return min
}

func lcm(a, b int) int {
	for i := 1; i <= a; i++ {
		if (i*b)%a == 0 {
			return i * b
		}
	}
	return -1
}

func solve(ls []string) string {
	v := newValley(ls)
	start := coord{0, 1}
	end := coord{v.h - 1, v.w - 2}
	min := v.trip(0, start, end)
	min = v.trip(min, end, start)
	min = v.trip(min, start, end)

	return strconv.Itoa(min)
}

func main() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("input2")
	fmt.Println(solve(ls))
}
