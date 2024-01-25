package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type coord struct {
	p int
	q int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")

	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))
	lines := make([][]coord, 0)
	mp := 0
	mq := 0
	for _, l := range ls {
		fmt.Println(l)
		sps := strings.Split(l, " -> ")
		line := make([]coord, 0)
		for _, cst := range sps {
			cpq := strings.Split(cst, ",")
			p, _ := strconv.Atoi(cpq[0])
			q, _ := strconv.Atoi(cpq[1])
			line = append(line, coord{
				p: p,
				q: q,
			})
			if p > mp {
				mp = p
			}
			if q > mq {
				mq = q
			}
		}
		lines = append(lines, line)
	}
	grid := make([]*[]byte, 0)
	for i := 0; i <= mp+200; i++ {
		col := make([]byte, 0)
		for j := 0; j <= mq+2; j++ {
			col = append(col, '.')
		}
		grid = append(grid, &col)
	}

	for _, line := range lines {
		st := line[0]
		for i := 1; i < len(line); i++ {
			block(grid, st, line[i])
			st = line[i]
		}
	}

	for _, col := range grid {
		(*col)[len(*col)-1] = '#'
	}

	out := false
	for {
		s := coord{500, 0}
		// fall
		for {
			if (*grid[s.p])[s.q+1] == '.' {
				s.q += 1
			} else if (*grid[s.p-1])[s.q+1] == '.' {
				s.p -= 1
				s.q += 1
			} else if (*grid[s.p+1])[s.q+1] == '.' {
				s.p += 1
				s.q += 1
			} else {
				(*grid[s.p])[s.q] = 's'
				if s.p == 500 && s.q == 0 {
					out = true
				}
				break
			}
		}

		if out {
			break
		}
	}

	sns := 0
	for _, col := range grid {
		for _, cell := range *col {
			if cell == 's' {
				sns++
			}
		}
	}

	fmt.Println(sns)
}

func block(grid []*[]byte, st coord, en coord) {
	if st.p == en.p {
		s, e := st.q, en.q
		if s > e {
			s, e = e, s
		}
		for i := s; i <= e; i++ {
			(*grid[st.p])[i] = '#'
		}
	} else {
		s, e := st.p, en.p
		if s > e {
			s, e = e, s
		}
		for i := s; i <= e; i++ {
			(*grid[i])[st.q] = '#'
		}
	}

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
