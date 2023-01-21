package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

type blueprint struct {
	n   int
	rbs [][]int
}

type items struct {
	cnt []int
}

func (its items) next(rbs []int) items {
	its.cnt = sliceAdd(its.cnt, rbs)
	return its
}

const (
	ORE int = iota
	CLAY
	OBS
	GEO
)

func (its items) make(bp blueprint, ty int) items {
	its.cnt = sliceMinus(its.cnt, bp.rbs[ty])
	return its
}

func (its items) hash() string {
	return strconv.Itoa(its.cnt[0]) + ":" +
		strconv.Itoa(its.cnt[1]) + ":" +
		strconv.Itoa(its.cnt[2]) + ":" +
		strconv.Itoa(its.cnt[3]) + ":"
}

func (its items) valid() bool {
	for _, v := range its.cnt {
		if v < 0 {
			return false
		}
	}
	return true
}

type builder struct {
	bp   blueprint
	rbs  []int
	maxt int
}

func (b *builder) hash(min int, its items) string {
	return strconv.Itoa(min) + "::" +
		strconv.Itoa(b.rbs[0]) + ":" +
		strconv.Itoa(b.rbs[1]) + ":" +
		strconv.Itoa(b.rbs[2]) + ":" +
		strconv.Itoa(b.rbs[3]) + "::" +
		its.hash()
}

type result struct {
	max    int
	maxtys [][]int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input1")
	time := 24

	bps := make([]blueprint, 0)
	for _, l := range ls {
		rx, _ := regexp.Compile("Blueprint (\\d*): Each ore robot costs (\\d*) ore. Each clay robot costs (\\d*) ore. Each obsidian robot costs (\\d*) ore and (\\d*) clay. Each geode robot costs (\\d*) ore and (\\d*) obsidian.")
		cs := rx.FindStringSubmatch(l)
		n, _ := strconv.Atoi(cs[1])
		orb, _ := strconv.Atoi(cs[2])
		crb, _ := strconv.Atoi(cs[3])
		obrbO, _ := strconv.Atoi(cs[4])
		obrbC, _ := strconv.Atoi(cs[5])
		grbO, _ := strconv.Atoi(cs[6])
		grbOb, _ := strconv.Atoi(cs[7])
		bps = append(bps, blueprint{
			n: n,
			rbs: [][]int{
				{orb, 0, 0, 0},
				{crb, 0, 0, 0},
				{obrbO, obrbC, 0, 0},
				{grbO, 0, grbOb, 0},
			},
		})
	}

	sum := 0
	for _, bp := range bps {
		bu := builder{
			bp:   bp,
			rbs:  []int{1, 0, 0, 0},
			maxt: time,
		}
		memo := map[string]result{}
		res := bu.maxGeode(0, newItems(), &memo, [][]int{})
		max := res.max
		fmt.Printf("bp: %d: max :%d\n", bp.n, max)
		printMaxtys(res.maxtys, bp)
		sum += (max * bp.n)
	}

	fmt.Println(sum)
}

func newItems() items {
	return items{cnt: []int{0, 0, 0, 0}}
}

func (b *builder) maxGeode(min int, its items, memo *map[string]result, tysnow [][]int) result {
	if min == b.maxt {
		return result{its.cnt[3], [][]int{}}
	}

	max := result{}
	h := b.hash(min, its)

	if v, fnd := (*memo)[h]; fnd {
		//fmt.Printf("depth%s: min %d memoized\n", depth, min)
		return v
	}

	// Optimization: Make geo if possible
	nits := its.make(b.bp, GEO)
	if nits.valid() {
		tys := []int{GEO}
		nits = nits.next(b.rbs)
		b.add(tys)
		max = b.maxGeode(min+1, nits, memo, append(tysnow, tys))
		b.reset(tys)

		max.maxtys = append([][]int{tys}, max.maxtys...)
		(*memo)[h] = max
		return max
	}

	is := []int{0, 1, 2, 4}
	for _, i := range is {
		nits := its
		tys := []int{}

		for e := 0; e < 3; e++ {
			if int(math.Pow(2, float64(e)))&i > 0 {
				nits = nits.make(b.bp, e)
				tys = append(tys, e)
			}
		}

		if nits.valid() {
			nits = nits.next(b.rbs)
			b.add(tys)
			maxres := b.maxGeode(min+1, nits, memo, append(tysnow, tys))
			b.reset(tys)

			if maxres.max > max.max {
				max = maxres
				max.maxtys = append([][]int{tys}, max.maxtys...)
			}
		}
	}

	(*memo)[h] = max
	return max
}

func isReqTys(tysnow [][]int) bool {
	return sliceEqual(tysnow, [][]int{
		{}, {}, {}, {1}, {1},
		{}, {1}, {}, {}, {},
		{2}, {1}, {}, {}, {2},
		{}, {}, {3}, {}, {}})
}

func (b *builder) add(tys []int) {
	for _, ty := range tys {
		b.rbs[ty]++
	}
}

func (b *builder) reset(tys []int) {
	for _, ty := range tys {
		b.rbs[ty]--
	}
}

func sliceAdd(a []int, b []int) (res []int) {
	for i := range a {
		res = append(res, a[i]+b[i])
	}
	return
}

func sliceMinus(a []int, b []int) (res []int) {
	for i := range a {
		res = append(res, a[i]-b[i])
	}
	return
}

func sliceEqual(a [][]int, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}

	for i := range a {
		if len(a[i]) != len(b[i]) {
			return false
		}
		for j := range a[i] {
			if a[i][j] != b[i][j] {
				return false
			}
		}
	}
	return true
}

func printMaxtys(maxtys [][]int, bp blueprint) {
	bots := []int{1, 0, 0, 0}
	its := []int{0, 0, 0, 0}
	for i, tys := range maxtys {
		for _, ty := range tys {
			its = sliceMinus(its, bp.rbs[ty])
		}
		its = sliceAdd(its, bots)
		for _, ty := range tys {
			bots[ty]++
		}
		fmt.Printf("%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%v\n", i+1, bots[0], its[0], bots[1], its[1], bots[2], its[2], bots[3], its[3], tys)
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
