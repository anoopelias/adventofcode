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
	path  *[]string
	ttt   int
	state int
}

const (
	FREE int = iota
	WALKING
	OPENING
)

func main() {

	inp := "input"
	mins := 26
	nrs := 2

	fmt.Println("Starting...")
	ls := linesOf(inp)
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

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
	rs := make([]*runner, 0)
	for i := 0; i < nrs; i++ {
		rp := make([]string, 0)
		rn := runner{vsm["AA"], &rp, 0, FREE}
		rs = append(rs, &rn)
	}

	mp := s.maxPressure(mins, rs)
	fmt.Println(mp)
	fmt.Println(*rs[0].path)
	fmt.Println(*rs[1].path)
}

func (s *solver) maxPressure(mins int, rs []*runner) int {
	var path []string

	if mins <= 0 {
		return 0
	}

	for _, rn := range rs {
		if rn.state == OPENING && rn.ttt == 0 {
			rn.state = FREE
			ps := mins * s.vs[rn.pos].pr
			return s.maxPressure(mins, rs) + ps
		}
	}

	for _, rn := range rs {
		if rn.state == WALKING && rn.ttt == 0 {
			rn.state = OPENING
			rn.ttt = 1
		}
	}
	for _, rn := range rs {
		if rn.state == FREE {
			pos := rn.pos
			max := 0
			sp := s.sps[rn.pos]
			for ni, nv := range s.vs {
				if !nv.open && nv.pr > 0 {
					rn.pos = ni
					s.vs[ni].open = true
					rn.state = WALKING
					rn.ttt = sp[ni]
					mx := s.maxPressure(mins, rs)
					s.vs[ni].open = false
					rn.state = FREE
					if max < mx {
						max = mx
						path = *rn.path
					}
				}
			}
			*rn.path = append([]string{s.vs[pos].name}, path...)
			return max
		}
	}

	ntick := 100
	for _, rn := range rs {
		if rn.ttt < ntick {
			ntick = rn.ttt
		}
	}

	for _, rn := range rs {
		rn.ttt -= ntick
	}

	return s.maxPressure(mins-ntick, rs)
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
