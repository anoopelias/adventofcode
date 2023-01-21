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
	bp   blueprint
	rbs  []int
	maxt int
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
	ls := linesOf("input1")
	time := 24

	bps := make([]blueprint, 0)
	for _, l := range ls {
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
	for _, bp := range bps {
		bu := builder{
			bp:   bp,
			rbs:  []int{1, 0, 0, 0},
			maxt: time,
		}
		memo := map[string]result{}
		res := bu.maxGeode(0, newItems(), &memo, []int{})
		max := res.max
		fmt.Printf("bp: %d: max :%d\n", bp.n, max)
		printMaxtys(res.maxtys, bp)
		out *= max
	}

	fmt.Println(out)
}

func newItems() items {
	return items{cnt: []int{0, 0, 0, 0}}
}

func (b *builder) maxGeode(min int, its items, memo *map[string]result, tysnow []int) result {
	if min == b.maxt {
		return result{its.cnt[3], tysnow}
	}

	max := result{}
	h := b.hash(min, its)

	if v, fnd := (*memo)[h]; fnd {
		//fmt.Printf("depth%s: min %d memoized\n", depth, min)
		return v
	}

	if isTysNow(tysnow) {
		fmt.Printf("min: %d h: %s\n", min, h)
	}

	is := []int{0, 1, 2, 3}

	if isTysNow(tysnow) {
		fmt.Printf("max0: %d %v\n", max.max, max.maxtys)
	}
	// no robots
	ms := b.maxt - min
	nrmax := (b.rbs[GEO] * ms) + its.cnt[GEO]
	if nrmax > max.max {
		max = result{
			max:    nrmax,
			maxtys: append(tysnow, getTys(ms)...),
		}
	}
	if isTysNow(tysnow) {
		fmt.Printf("max1: %d %v\n", max.max, max.maxtys)
	}
	for _, ty := range is {
		ms := b.timeToTy(its, ty)
		if ms > 0 && ms < b.maxt-min {
			nits := its.next(b.rbs, ms)
			nits = nits.make(b.bp, ty)
			b.rbs[ty]++
			if isTysNow(tysnow) {
				fmt.Printf("ty: %d ms: %d\n", ty, ms)
				fmt.Printf("max2: %d %v\n", max.max, max.maxtys)
			}
			maxgeo := b.maxGeode(min+ms, nits, memo, append(tysnow, getTysTy(ms, ty)...))
			if isTysNow(tysnow) {
				fmt.Printf("ty: %d max: %d\n", ty, maxgeo.max)
				fmt.Printf("max3: %d %v\n", max.max, max.maxtys)
			}
			b.rbs[ty]--
			if maxgeo.max > max.max {
				if isTysNow(tysnow) {
					fmt.Printf("changing: %d %d\n", maxgeo.max, max.max)
					fmt.Printf("max4: %d %v\n", max.max, max.maxtys)
				}
				max = maxgeo
			}
		}
	}

	if isTysNow(tysnow) {
		fmt.Printf("max5: %d %v\n", max.max, max.maxtys)
	}

	//(*memo)[h] = max
	return max
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

func getTys(ms int) []int {
	tys := []int{}
	for i := 0; i < ms; i++ {
		tys = append(tys, -1)
	}
	return tys
}

func getTysTy(ms int, ty int) []int {
	tys := []int{}
	for i := 0; i < ms-1; i++ {
		tys = append(tys, -1)
	}
	tys = append(tys, ty)
	return tys
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
