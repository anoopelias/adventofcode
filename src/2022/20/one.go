package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type node struct {
	num  int
	next *node
	prev *node
}

func (n *node) mix(h *node) *node {

	if n.num == 0 {
		return h
	}

	nh := h
	n.prev.next = n.next
	n.next.prev = n.prev

	tgt := n.nth(n.num)
	if n.num < 0 {
		tgt = tgt.prev
	}

	if h == n {
		nh = n.next
	} else if h == tgt.prev {
		nh = n
	}

	tmp := tgt.next
	tgt.next = n
	n.next = tmp
	tmp.prev = n
	n.prev = tgt

	return nh
}

func (n *node) nth(num int) *node {
	if num < 0 {
		return n.back(num * -1)
	}
	return n.fwd(num)
}

func (n *node) fwd(num int) *node {
	node := n
	for num != 0 {
		num--
		node = node.next
	}
	return node
}

func (n *node) back(num int) *node {
	node := n
	for num != 0 {
		num--
		node = node.prev
	}
	return node
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input1")
	ns := make([]*node, len(ls))

	hnum, _ := strconv.Atoi(ls[0])
	nh := node{num: hnum}
	h := &nh
	p := h
	ln := len(ls)
	ns[0] = h
	var zr *node

	for i := 1; i < ln; i++ {
		num, _ := strconv.Atoi(ls[i])
		n := node{num: num}
		ns[i] = &n
		n.prev = p
		p = &n
		n.prev.next = p

		if num == 0 {
			zr = p
		}
	}
	p.next = h
	h.prev = p

	print(h, ln)
	for _, n := range ns {
		h = n.mix(h)
		print(h, ln)
	}

	x := zr.nth(1000).num
	y := zr.nth(2000).num
	z := zr.nth(3000).num

	fmt.Printf("%d %d %d %d\n", x, y, z, x+y+z)
}

func print(nd *node, ln int) {
	nums := make([]string, 0)
	for ln != 0 {
		ln--
		nums = append(nums, strconv.Itoa(nd.num))
		nd = nd.next
	}
	fmt.Println(strings.Join(nums, "\t"))
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
