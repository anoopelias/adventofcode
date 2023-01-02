package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type valve struct {
	name   string
	pr     int
	open   bool
	marked bool
	adj    []int
}

type node struct {
	nd   int
	dist int
}

type solver struct {
	vs  []*valve
	sps map[int]map[int]int
}

type runner struct {
	pos   int
	path  []string
	pathn []pathnode
	ttt   int
	state int
}

type pathnode struct {
	pos  int
	mins int
	pr   int
}

const (
	FREE int = iota
	WALKING
	OPENING
)

func main() {

	inp := "input2"
	mins := 30

	fmt.Println("Starting...")
	ls := linesOf(inp)

	m := make(map[string][]string)
	vs := make([]*valve, 0)
	vsm := make(map[string]int)

	for i := 0; i < len(ls); i++ {
		rx, _ := regexp.Compile("Valve (..) has flow rate=([0-9]+); tunnel(s)? lead(s)? to valve(s)? (.*)")
		cs := rx.FindStringSubmatch(ls[i])
		name := cs[1]
		pr, _ := strconv.Atoi(cs[2])
		tg := strings.Split(cs[6], ", ")

		vs = append(vs, &valve{
			name: name,
			pr:   pr,
		})
		num := len(vs) - 1
		vsm[name] = num

		m[name] = tg
	}

	for _, v := range vs {
		v.adj = make([]int, 0)
		for _, ad := range m[v.name] {
			v.adj = append(v.adj, vsm[ad])
		}
	}

	sps := make(map[int]map[int]int)

	for f := range vs {
		sps[f] = make(map[int]int, 0)
		for t := range vs {
			sps[f][t] = shortestPath(vs, f, t)
		}
	}

	s := solver{vs, sps}

	rp := make([]string, 0)
	rpns := make([]pathnode, 0)
	rn := runner{vsm["AA"], rp, rpns, 0, FREE}

	mp := s.maxPressure(mins, &rn)
	fmt.Println(mp)
	fmt.Println(rn.path)
	printPathNodes(vs, rn.pathn, mins)
}

func printPathNodes(vs []*valve, pns []pathnode, mins int) {
	for _, pn := range pns {
		fmt.Printf("%s \t%d \t%d \t%d\n", vs[pn.pos].name,
			mins-pn.mins, pn.pr, pn.mins*pn.pr)
	}
}

func (s *solver) maxPressure(mins int, rn *runner) int {
	pos := rn.pos
	v := s.vs[pos]
	max := 0
	path := make([]string, 0)
	sp := s.sps[pos]
	ot := 0
	if v.pr > 0 {
		v.open = true
		ot = 1
	}
	for ni, nv := range s.vs {
		tn := sp[ni] + ot
		if !nv.open && nv.pr > 0 && mins > tn {
			rn.pos = ni
			mx := s.maxPressure(mins-tn, rn)
			if max < mx {
				max = mx
				path = rn.path
			}
		}
	}
	v.open = false
	rn.path = append([]string{s.vs[pos].name}, path...)
	return max + ((mins - 1) * v.pr)
}

func shortestPath(vs []*valve, from int, to int) int {
	mrkd := make(map[int]bool)
	qu := []*node{{from, 0}}

	for len(qu) > 0 {
		h, q := qu[0], qu[1:]
		qu = q
		_, has := mrkd[h.nd]
		if !has {
			mrkd[h.nd] = true
			if h.nd == to {
				return h.dist
			}

			for _, nd := range vs[h.nd].adj {
				qu = append(qu, &node{nd, h.dist + 1})
			}
		}
	}
	return -1
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
