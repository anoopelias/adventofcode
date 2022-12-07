package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Dir struct {
	name string
	dirs []*Dir
	file []int
	p    *Dir
}

func main() {
	//fmt.Println("Starting...")
	ls := linesOf("input2")[1:]
	root := Dir{
		dirs: make([]*Dir, 0),
		file: make([]int, 0),
	}
	curr := &root

	for _, l := range ls {
		//fmt.Println(l)
		if l[0] == '$' {
			splits := strings.Split(l, " ")
			if splits[1] == "cd" {
				dn := splits[2]
				if dn == ".." {
					curr = curr.p
				} else {
					sd := hasDir(curr, dn)
					if sd == nil {
						sd = &Dir{
							name: dn,
							dirs: make([]*Dir, 0),
							file: make([]int, 0),
							p:    curr,
						}
						curr.dirs = append(curr.dirs, sd)
					}
					curr = sd
				}
			}
		} else {
			splits := strings.Split(l, " ")
			if splits[0] == "dir" {
				dn := splits[1]
				sd := hasDir(curr, dn)
				if sd == nil {
					sd = &Dir{
						name: dn,
						dirs: make([]*Dir, 0),
						file: make([]int, 0),
						p:    curr,
					}
					curr.dirs = append(curr.dirs, sd)
				}
			} else {
				len, _ := strconv.Atoi(splits[0])
				curr.file = append(curr.file, len)
			}
		}
	}
	_, r := sizeOf(&root)
	fmt.Println(r)

}

func sizeOf(d *Dir) (int, int) {
	s := 0
	r := 0
	for _, f := range d.file {
		s += f
	}
	for _, d := range d.dirs {
		ds, rs := sizeOf(d)
		s += ds
		r += rs
	}

	if s < 100000 {
		r += s
	}
	return s, r
}

func hasDir(d *Dir, n string) *Dir {
	for _, sd := range d.dirs {
		if sd.name == n {
			return sd
		}
	}
	return nil
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
