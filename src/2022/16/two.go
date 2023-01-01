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
	adj    []string
	open   bool
	marked bool
}

type node struct {
	nd   string
	dist int
}

type solver struct {
	m   map[string]*valve
	sps map[string]map[string]int
}

type runner struct {
	pos   string
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
	fmt.Println("Starting...")
	ls := linesOf("input")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	m := make(map[string]*valve)

	for i := 0; i < len(ls); i++ {
		rx, _ := regexp.Compile("Valve (..) has flow rate=([0-9]+); tunnel(s)? lead(s)? to valve(s)? (.*)")
		cs := rx.FindStringSubmatch(ls[i])
		name := cs[1]
		pr, _ := strconv.Atoi(cs[2])
		tg := strings.Split(cs[6], ", ")

		m[name] = &valve{
			name: name,
			pr:   pr,
			adj:  tg,
		}
	}
	sps := make(map[string]map[string]int)

	for fs := range m {
		sps[fs] = make(map[string]int)
		for ts := range m {
			sps[fs][ts] = shortestPath(m, fs, ts)
		}
	}

	s := solver{m, sps}
	p1p := make([]string, 0)
	p2p := make([]string, 0)
	p1 := runner{"AA", &p1p, 0, FREE}
	p2 := runner{"AA", &p2p, 0, FREE}

	mp := s.maxPressure(&p1, &p2, 30)
	fmt.Println(mp)
	fmt.Println(*p1.path)
}

func (s *solver) maxPressure(p1 *runner, p2 *runner, mins int) int {
	if mins <= 0 {
		return 0
	} else if p1.state == WALKING {
		p1.state = OPENING
		return s.maxPressure(p1, p2, mins-1)
	} else if p1.state == OPENING {
		p1.state = FREE
		ps := mins * s.m[p1.pos].pr
		return s.maxPressure(p1, p2, mins) + ps
	} else if p1.state == FREE {
		pos := p1.pos
		max := 0
		path := make([]string, 0)
		sp := s.sps[p1.pos]
		for nvs := range s.m {
			nv := s.m[nvs]
			if !nv.open && nv.pr > 0 {
				p1.pos = nvs
				s.m[nvs].open = true
				p1.state = WALKING
				mx := s.maxPressure(p1, p2, mins-sp[nvs])
				s.m[nvs].open = false
				if max < mx {
					max = mx
					path = *p1.path
				}
			}
		}
		*p1.path = append([]string{pos}, path...)
		return max
	}
	return 0
}

func shortestPath(m map[string]*valve, from string, to string) int {
	mrkd := make(map[string]bool)
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

			v, _ := m[h.nd]
			for _, nd := range v.adj {
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
