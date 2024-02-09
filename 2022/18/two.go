package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type pos struct {
	x, y, z int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/18/input2")

	ps := make([]pos, 0)
	m := make(map[pos]bool)
	mkd := make(map[pos]bool)

	maxx := 0
	maxy := 0
	maxz := 0

	for _, l := range ls {
		sps := strings.Split(l, ",")
		x, _ := strconv.Atoi(sps[0])
		y, _ := strconv.Atoi(sps[1])
		z, _ := strconv.Atoi(sps[2])
		p := pos{x, y, z}
		ps = append(ps, p)
		m[p] = true

		if x > maxx {
			maxx = x
		}
		if y > maxy {
			maxy = y
		}
		if z > maxz {
			maxz = z
		}
	}

	q := []pos{{0, 0, 0}}

	for len(q) != 0 {
		h := q[0]
		q = q[1:]
		if !mkd[h] {
			mkd[h] = true
			nbs := getNighbors(h, m, maxx, maxy, maxz)

			for _, nb := range nbs {
				q = append(q, nb)
			}
		}
	}
	cnt := 0
	for _, p := range ps {
		cnt += openFaces(p, mkd)
	}
	fmt.Println(cnt)
}

func getNighbors(p pos, m map[pos]bool, maxx, maxy, maxz int) []pos {
	nbs := []pos{}

	ps := []pos{
		{p.x, p.y, p.z + 1},
		{p.x, p.y, p.z - 1},
		{p.x, p.y + 1, p.z},
		{p.x, p.y - 1, p.z},
		{p.x + 1, p.y, p.z},
		{p.x - 1, p.y, p.z},
	}
	for _, p := range ps {
		if p.x >= -1 && p.y >= -1 && p.z >= -1 &&
			p.x <= (maxx+1) && p.y <= (maxy+1) && p.z <= (maxz+1) &&
			!m[p] {
			nbs = append(nbs, p)
		}
	}
	return nbs
}

func openFaces(p pos, mkd map[pos]bool) (cnt int) {
	ps := []pos{
		{p.x, p.y, p.z + 1},
		{p.x, p.y, p.z - 1},
		{p.x, p.y + 1, p.z},
		{p.x, p.y - 1, p.z},
		{p.x + 1, p.y, p.z},
		{p.x - 1, p.y, p.z},
	}

	for _, p := range ps {
		if mkd[p] {
			cnt++
		}
		// } else {
		// 	fmt.Printf("Unmarked %v\n", p)
		// }
	}
	return cnt
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
