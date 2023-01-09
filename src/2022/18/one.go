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
	ls := linesOf("input2")

	ps := make([]pos, 0)
	m := make(map[string]bool)

	for _, l := range ls {
		sps := strings.Split(l, ",")
		x, _ := strconv.Atoi(sps[0])
		y, _ := strconv.Atoi(sps[1])
		z, _ := strconv.Atoi(sps[2])
		ps = append(ps, pos{x, y, z})
		m[key(x, y, z)] = true
	}

	fs := 0
	for _, p := range ps {
		fs += countOpenFaces(p, m)
	}
	fmt.Println(fs)
}

func key(x, y, z int) string {
	return strconv.Itoa(x) + " " + strconv.Itoa(y) + " " + strconv.Itoa(z)
}

func countOpenFaces(p pos, m map[string]bool) int {
	cnt := 0

	if !m[key(p.x, p.y, p.z+1)] {
		cnt++
	}
	if !m[key(p.x, p.y, p.z-1)] {
		cnt++
	}
	if !m[key(p.x, p.y+1, p.z)] {
		cnt++
	}
	if !m[key(p.x, p.y-1, p.z)] {
		cnt++
	}
	if !m[key(p.x+1, p.y, p.z)] {
		cnt++
	}
	if !m[key(p.x-1, p.y, p.z)] {
		cnt++
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
