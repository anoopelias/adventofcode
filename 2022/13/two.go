package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

type item struct {
	num    int
	list   []*item
	isList bool
}

type packet struct {
	items []*item
	i     int
}

const (
	CONTINUE int = iota
	OUT_OF_ORDER
	IN_ORDER
)

func main() {
	fmt.Println("Starting...")
	lns := linesOf("../../../aoc-files/2022/13/input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(lns)))

	ps := make([]*packet, 0)
	for i := 0; i < len(lns); i += 3 {
		ls := lns[i]
		rs := lns[i+1]
		l, _ := parse(ls, 0)
		r, _ := parse(rs, 0)
		ps = append(ps, &packet{items: l})
		ps = append(ps, &packet{items: r})
	}

	div1, _ := parse("[[2]]", 0)
	div2, _ := parse("[[6]]", 0)
	p1 := &packet{items: div1}
	p2 := &packet{items: div2}
	ps = append(ps, p1)
	ps = append(ps, p2)

	sort.SliceStable(ps, func(i, j int) bool {
		if test(ps[i].items, ps[j].items) == IN_ORDER {
			return true
		}
		return false
	})

	for i, p := range ps {
		p.i = i + 1
	}

	fmt.Println(p1.i * p2.i)
}

func test(left []*item, right []*item) int {
	var i int
	var r *item

	if len(right) == 0 && len(left) > 0 {
		return OUT_OF_ORDER
	}

	for i, r = range right {
		if i > len(left)-1 {
			return IN_ORDER
		} else if left[i].isList && r.isList {
			res := test(left[i].list, r.list)
			if res == CONTINUE {
				continue
			}
			return res
		} else if left[i].isList && !r.isList {
			res := test(left[i].list, []*item{r})
			if res == CONTINUE {
				continue
			}
			return res

		} else if !left[i].isList && r.isList {
			res := test([]*item{left[i]}, r.list)
			if res == CONTINUE {
				continue
			}
			return res
		} else if left[i].num < r.num {
			return IN_ORDER
		} else if left[i].num > r.num {
			return OUT_OF_ORDER
		}
	}

	if len(left)-1 > i {
		return OUT_OF_ORDER
	}

	return CONTINUE
}

func parse(s string, n int) ([]*item, int) {
	items := make([]*item, 0)
	cur := ""
	var i int
	for i = n; i <= len(s); i++ {
		if i == len(s) {
			if len(cur) != 0 {
				n, _ := strconv.Atoi(cur)
				items = append(items, &item{
					num: n,
				})
			}
		} else if s[i] == '[' {
			lst, ni := parse(s, i+1)
			i = ni
			items = append(items, &item{
				list:   lst,
				isList: true,
			})
		} else if s[i] == ']' {
			if len(cur) != 0 {
				num, _ := strconv.Atoi(cur)
				items = append(items, &item{
					num: num,
				})
			}
			break
		} else if s[i] == ',' {
			if len(cur) != 0 {
				num, _ := strconv.Atoi(cur)
				items = append(items, &item{
					num: num,
				})
				cur = ""
			}
		} else {
			cur += string(s[i])
		}
	}
	return items, i
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
