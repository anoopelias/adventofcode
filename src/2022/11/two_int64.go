package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type monkey struct {
	items []int
	op    func(int) int
	tst   func(int) bool
	tr    int
	fl    int
	ins   int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	ms := []*monkey{}
	fac := 1
	for i := 0; i < len(ls); i++ {
		i++
		sis := strings.Split(strings.Split(ls[i], ":")[1], ",")
		items := []int{}
		for _, is := range sis {
			i, _ := strconv.Atoi(strings.Trim(is, " "))
			items = append(items, i)
		}
		i++
		op := getOpFunc(ls[i])
		i++
		sps := strings.Split(ls[i], " ")
		div, _ := strconv.Atoi(sps[len(sps)-1])
		fac *= div
		tst := func(i int) bool {
			return i%div == 0
		}
		i++
		sps = strings.Split(ls[i], " ")
		tr, _ := strconv.Atoi(sps[len(sps)-1])
		i++
		sps = strings.Split(ls[i], " ")
		fl, _ := strconv.Atoi(sps[len(sps)-1])

		m := &monkey{
			items: items,
			op:    op,
			tst:   tst,
			tr:    tr,
			fl:    fl,
		}
		ms = append(ms, m)
		i++
	}

	for i := 0; i < 10000; i++ {
		round(&ms, fac)
	}
	fmt.Printf("%v", ms)
}

func round(ms *[]*monkey, fac int) {
	for i := range *ms {
		turn(ms, i, fac)
	}
}

func turn(msp *[]*monkey, i int, fac int) {
	ms := *msp
	m := ms[i]
	for len(m.items) > 0 {
		m.ins++
		it, its := m.items[0], m.items[1:]
		m.items = its
		w := m.op(it)
		w = w % fac
		// w = w / 3
		if m.tst(w) {
			ms[m.tr].items = append(ms[m.tr].items, w)
		} else {
			ms[m.fl].items = append(ms[m.fl].items, w)
		}
	}
}

func getOpFunc(str string) func(int) int {
	exp := strings.Split(strings.Trim(str, " "), " ")

	return func(i int) int {
		opnd := i
		if exp[len(exp)-1] != "old" {
			ni, _ := strconv.Atoi(exp[len(exp)-1])
			opnd = ni
		}
		switch exp[len(exp)-2] {
		case "+":
			return i + opnd
		case "*":
			return i * opnd
		}
		return 0
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
