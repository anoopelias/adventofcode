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
	n     int
	orb   int
	crb   int
	obrbO int
	obrbC int
	grbO  int
	grbOb int
}

type items struct {
	ore  int
	clay int
	obs  int
	geo  int
}

func (its items) next(b builder) items {
	its.ore += b.orbs
	its.clay += b.crbs
	its.obs += b.obrbs
	its.geo += b.grbs
	return its
}

const (
	ORE int = iota
	CLAY
	OBS
	GEO
)

func (its items) make(b builder, ty int) items {
	switch ty {
	case ORE:
		its.ore -= b.bp.orb
	case CLAY:
		its.ore -= b.bp.crb
	case OBS:
		its.ore -= b.bp.obrbO
		its.clay -= b.bp.obrbC
	case GEO:
		its.ore -= b.bp.grbO
		its.obs -= b.bp.grbOb
	}
	return its
}

func (its items) hash() string {
	return strconv.Itoa(its.ore) + ":" +
		strconv.Itoa(its.clay) + ":" +
		strconv.Itoa(its.obs) + ":" +
		strconv.Itoa(its.geo) + ":"
}

func (its items) valid() bool {
	return its.ore >= 0 &&
		its.clay >= 0 &&
		its.obs >= 0 &&
		its.geo >= 0
}

type builder struct {
	bp    blueprint
	orbs  int
	crbs  int
	obrbs int
	grbs  int
	maxt  int
}

func (b *builder) hash(min int, its items) string {
	return strconv.Itoa(min) + "::" +
		strconv.Itoa(b.orbs) + ":" +
		strconv.Itoa(b.crbs) + ":" +
		strconv.Itoa(b.obrbs) + ":" +
		strconv.Itoa(b.grbs) + "::" +
		its.hash()
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input")
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
		bps = append(bps, blueprint{n, orb, crb, obrbO, obrbC, grbO, grbOb})
	}

	sum := 0
	for _, bp := range bps {
		bu := builder{
			bp:   bp,
			orbs: 1,
			maxt: time,
		}
		memo := map[string]int{}
		max := bu.maxGeode(0, items{}, &memo, "")
		fmt.Printf("bp: %d: max :%d\n", bp.n, max)
		sum += (max * bp.n)
	}

	fmt.Println(sum)
}

func (b *builder) maxGeode(min int, its items, memo *map[string]int, depth string) int {
	if min == b.maxt {
		return its.geo
	}

	h := b.hash(min, its)

	if v, fnd := (*memo)[h]; fnd {
		//fmt.Printf("depth%s: min %d memoized\n", depth, min)
		return v
	}

	max := 0
	for i := 0; i < 8; i++ {
		nits := its
		tys := []int{}

		// Optimization: Make geo if possible
		nits = nits.make(*b, GEO)
		if !nits.valid() {
			nits = its
		} else {
			tys = append(tys, GEO)
		}

		for e := 0; e < 3; e++ {
			if int(math.Pow(2, float64(e)))&i > 0 {
				nits = nits.make(*b, e)
				tys = append(tys, e)
			}
		}

		if nits.valid() {
			nits = nits.next(*b)
			b.add(tys)
			ngeo := b.maxGeode(min+1, nits, memo, depth+" "+strconv.Itoa(min))
			if min == 4 {
				fmt.Printf("depth%s: min %d, tys %v max %d\n", depth, min, tys, ngeo)
			}
			b.reset(tys)

			if ngeo > max {
				max = ngeo
				if min == 4 {
					fmt.Printf("depth%s: min %d, tys %v [max updated]:%d hash%s\n", depth, min, tys, max, h)
				}
			}
		}
	}

	(*memo)[h] = max
	return max
}

func (b *builder) add(tys []int) {
	for _, ty := range tys {
		switch ty {
		case ORE:
			b.orbs++
		case CLAY:
			b.crbs++
		case OBS:
			b.obrbs++
		case GEO:
			b.grbs++
		}
	}
}

func (b *builder) reset(tys []int) {
	for _, ty := range tys {
		switch ty {
		case ORE:
			b.orbs--
		case CLAY:
			b.crbs--
		case OBS:
			b.obrbs--
		case GEO:
			b.grbs--
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
