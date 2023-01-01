package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type valve struct {
	name string
	pr   int
	adj  []string
	open bool
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
	pos  string
	path *[]string
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
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
	p1 := runner{"AA", &p1p}

	mp := s.maxPressure(&p1, 30)
	fmt.Println(mp)
	fmt.Println(*p1.path)
}

func (s *solver) maxPressure(p1 *runner, mins int) int {
	if mins == 0 {
		return 0
	}

	v := s.m[p1.pos]
	if !v.open {
		pos := p1.pos
		max := 0
		path := make([]string, 0)
		sp := s.sps[p1.pos]
		ot := 0
		v.open = true
		if v.pr > 0 {
			ot = 1
		}
		for nvs := range s.m {
			nv := s.m[nvs]
			tn := sp[nvs] + ot
			if !nv.open && nv.pr > 0 && mins > tn {
				p1.pos = nvs
				mx := s.maxPressure(p1, mins-tn)
				if max < mx {
					max = mx
					path = *p1.path
				}
			}
		}
		v.open = false
		*p1.path = append([]string{pos}, path...)
		return max + ((mins - 1) * v.pr)
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
