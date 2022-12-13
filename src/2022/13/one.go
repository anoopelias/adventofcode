package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type item struct {
	num    int
	list   []*item
	isList bool
}

type pair struct {
	left  []*item
	right []*item
}

const (
	CONTINUE int = iota
	OUT_OF_ORDER
	IN_ORDER
)

func main() {
	fmt.Println("Starting...")
	lns := linesOf("input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(lns)))

	pairs := make([]pair, 0)
	for i := 0; i < len(lns); i += 3 {
		ls := lns[i]
		rs := lns[i+1]
		l, _ := parse(ls, 0)
		r, _ := parse(rs, 0)
		pairs = append(pairs, pair{
			left:  l,
			right: r,
		})
	}

	sum := 0
	for i, pair := range pairs {
		res := test(pair.left, pair.right)
		if res == IN_ORDER {
			sum += (i + 1)
		}
	}
	fmt.Println(sum)
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
