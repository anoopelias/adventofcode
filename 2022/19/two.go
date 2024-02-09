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

func (its items) next(rbs []int, min int) items {
	its.cnt = sliceMultAdd(its.cnt, rbs, min)
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

func (its items) valid() bool {
	for _, v := range its.cnt {
		if v < 0 {
			return false
		}
	}
	return true
}

type builder struct {
	bp     blueprint
	rbs    []int
	maxt   int
	limits []int
}

func (b *builder) hash(min int, its items) string {
	return strconv.Itoa(min) + "::" +
		strconv.Itoa(b.rbs[0]) + ":" + strconv.Itoa(its.cnt[0]) + "::" +
		strconv.Itoa(b.rbs[1]) + ":" + strconv.Itoa(its.cnt[1]) + "::" +
		strconv.Itoa(b.rbs[2]) + ":" + strconv.Itoa(its.cnt[2]) + "::" +
		strconv.Itoa(b.rbs[3]) + ":" + strconv.Itoa(its.cnt[3])
}

type result struct {
	max    int
	maxtys []int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("../../../aoc-files/2022/19/input2")

	// https://github.com/bhosale-ajay/adventofcode/blob/62e20bb6addbd31d8993b41e2910a6c6736c1c56/2022/ts/D19.test.ts#L121
	//time := 24
	//limits := []int{16, 6, 3, 2}
	time := 32
	limits := []int{16, 6, 3, 1}

	// 	bp: 1: max :27
	// bp: 2: max :38
	// bp: 3: max :43
	// 44118

	// 	bp: 1: max :27
	// bp: 2: max :38
	// bp: 3: max :44
	// 45144

	// 	bp: 1: max :28
	// bp: 2: max :38
	// bp: 3: max :44
	// 46816

	bps := make([]blueprint, 0)
	for _, l := range ls[:3] {
		rx, _ := regexp.Compile("Blueprint (\\d*): " +
			"Each ore robot costs (\\d*) ore. " +
			"Each clay robot costs (\\d*) ore. " +
			"Each obsidian robot costs (\\d*) ore and (\\d*) clay. " +
			"Each geode robot costs (\\d*) ore and (\\d*) obsidian.")
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

	out := 1
	tysnow := make([]int, time)
	for i := 0; i < len(tysnow); i++ {
		tysnow[i] = -1
	}

	for _, bp := range bps {
		bu := builder{
			bp:     bp,
			rbs:    []int{1, 0, 0, 0},
			maxt:   time,
			limits: limits,
		}
		memo := map[string]int{}
		//tysnow := make([]int, time)
		max := bu.maxGeode(0, newItems(), &memo)
		fmt.Printf("bp: %d: max :%d\n", bp.n, max)
		//printMaxtys(tysnow, bp)
		out *= max
	}

	fmt.Println(out)
}

func newItems() items {
	return items{cnt: []int{0, 0, 0, 0}}
}

func (b *builder) maxGeode(min int, its items, memo *map[string]int) int {
	if min == b.maxt {
		return its.cnt[3]
	}

	maxx := 0
	h := b.hash(min, its)

	if v, fnd := (*memo)[h]; fnd {
		return v
	}

	is := []int{0, 1, 2, 3}

	// no robots
	rt := b.maxt - min
	nrmax := (b.rbs[GEO] * rt) + its.cnt[GEO]
	if nrmax > maxx {
		maxx = nrmax
	}
	for _, ty := range is {
		ms := b.timeToTy(its, ty)
		if rt >= b.limits[ty] && ms > 0 && ms < b.maxt-min {
			nits := its.next(b.rbs, ms)
			nits = nits.make(b.bp, ty)
			nmin := min + ms
			b.rbs[ty]++
			//(*tysnow)[nmin-1] = ty
			maxgeo := b.maxGeode(nmin, nits, memo)
			//(*tysnow)[nmin-1] = -1
			b.rbs[ty]--
			if maxgeo > maxx {
				maxx = maxgeo
			}
		}
	}

	(*memo)[h] = maxx
	return maxx
}

func (b *builder) timeToTy(its items, ty int) int {
	its = its.make(b.bp, ty)
	time := 0

	for t, it := range its.cnt {
		if it < 0 {
			it *= -1
			if b.rbs[t] == 0 {
				return -1
			}
			ti := int(math.Ceil(float64(it) / float64(b.rbs[t])))

			if ti > time {
				time = ti
			}
		}
	}

	return time + 1
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

func sliceCopy(s []int) []int {
	c := make([]int, len(s))
	copy(c, s)
	return c
}

func sliceAdd(a []int, b []int) (res []int) {
	for i := range a {
		res = append(res, a[i]+b[i])
	}
	return
}

func sliceMultAdd(a []int, b []int, mult int) (res []int) {
	for i := range a {
		res = append(res, a[i]+(b[i]*mult))
	}
	return
}

func sliceMinus(a []int, b []int) (res []int) {
	for i := range a {
		res = append(res, a[i]-b[i])
	}
	return
}

func sliceEqual(a []int, b []int) bool {
	if len(a) != len(b) {
		return false
	}

	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func printMaxtys(maxtys []int, bp blueprint) {
	bots := []int{1, 0, 0, 0}
	its := []int{0, 0, 0, 0}
	for i, ty := range maxtys {
		if ty != -1 {
			its = sliceMinus(its, bp.rbs[ty])
		}
		its = sliceAdd(its, bots)
		if ty != -1 {
			bots[ty]++
		}
		fmt.Printf("%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\n", i+1, bots[0], its[0], bots[1], its[1], bots[2], its[2], bots[3], its[3], ty)
	}
}

func isTysNow(tys []int) bool {
	tgt := []int{
		-1,
		-1,
		1,
		-1,
		1,
		-1,
		1,
		-1,
		-1,
		-1,
		2,
		1,
		-1,
		-1,
		2,
		-1,
		-1,
		3,
		-1,
		-1,
		3,
	}
	return sliceEqual(tys, tgt)
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
