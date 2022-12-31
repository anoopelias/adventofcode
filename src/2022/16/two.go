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
	adj  *[]string
	open bool
}

type node struct {
	nd   string
	dist int
}

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
			adj:  &tg,
		}
	}
	sps := make(map[string]map[string]int)

	for fs := range m {
		sps[fs] = make(map[string]int)
		for ts := range m {
			sps[fs][ts] = shortestPath(m, fs, ts)
		}
	}

	mp, path := maxPressure(m, sps, "AA", 26)
	fmt.Println(mp)
	fmt.Println(path)
}

func maxPressure(m map[string]*valve, sps map[string]map[string]int, start string, mins int) (int, []string) {
	if mins == 0 {
		return 0, []string{}
	}

	v := m[start]
	if !v.open {
		max := 0
		path := make([]string, 0)
		sp := sps[start]
		ot := 0
		if v.pr > 0 {
			v.open = true
			ot = 1
		}
		for nvs := range m {
			nv := m[nvs]
			tn := sp[nvs] + ot
			if !nv.open && nv.pr > 0 && mins > tn {
				mx, p := maxPressure(m, sps, nvs, mins-tn)
				if max < mx {
					max = mx
					path = p
				}
			}
		}
		v.open = false
		path = append([]string{start}, path...)
		return max + ((mins - 1) * v.pr), path
	}

	return 0, []string{}
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
			for _, nd := range *v.adj {
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
