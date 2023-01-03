package main

import (
	"fmt"
	"io/ioutil"
	"math"
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

func main() {

	inp := "input2"
	mins := 26

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

	nis := nodes(vs)
	max := 0
	partitions(nis, func(p1, p2 []int) {
		ps1 := s.maxPressure(mins, vsm["AA"], p1)
		ps2 := s.maxPressure(mins, vsm["AA"], p2)

		mx := ps1 + ps2
		if max < mx {
			max = mx
			fmt.Printf("Updating max %d\n", max)
		}
	})
	fmt.Println(max)
}

func nodes(vs []*valve) (nis []int) {
	for ni, nv := range vs {
		if nv.pr > 0 {
			nis = append(nis, ni)
		}
	}
	return
}

func partitions(nis []int, f func([]int, []int)) {
	max := int(math.Pow(2, float64(len(nis)))) - 1

	for i := 0; i <= max; i++ {

		p1 := make([]int, 0)
		p2 := make([]int, 0)
		for j := 0; j < len(nis); j++ {
			pos := int(math.Pow(2, float64(j)))
			if i&pos == 0 {
				p1 = append(p1, nis[j])
			} else {
				p2 = append(p2, nis[j])
			}
		}
		f(p1, p2)
	}

}

func (s *solver) maxPressure(mins int, pos int, vis []int) int {
	v := s.vs[pos]
	max := 0
	sp := s.sps[pos]
	ot := 0
	if v.pr > 0 {
		v.open = true
		ot = 1
	}

	for _, ni := range vis {
		nv := s.vs[ni]
		tn := sp[ni] + ot
		if !nv.open && nv.pr > 0 && mins > tn {
			mx := s.maxPressure(mins-tn, ni, vis)
			if max < mx {
				max = mx
			}
		}
	}
	v.open = false
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
