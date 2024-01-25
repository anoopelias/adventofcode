package main

import (
	"fmt"
	"io/ioutil"
	"math/big"
	"strconv"
	"strings"
)

type monkey struct {
	items []big.Int
	op    func(*big.Int)
	tst   func(*big.Int) bool
	tr    int
	fl    int
	ins   int
}

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input")
	fmt.Println("No of lines: " + strconv.Itoa(len(ls)))

	ms := []*monkey{}
	for i := 0; i < len(ls); i++ {
		i++
		sis := strings.Split(strings.Split(ls[i], ":")[1], ",")
		items := []big.Int{}
		for _, is := range sis {
			i, _ := strconv.Atoi(strings.Trim(is, " "))
			items = append(items, *big.NewInt(int64(i)))
		}
		i++
		op := getOpFunc(ls[i])
		i++
		sps := strings.Split(ls[i], " ")
		div, _ := strconv.Atoi(sps[len(sps)-1])
		bdiv := big.NewInt(int64(div))
		tst := func(w *big.Int) bool {
			ni := new(big.Int)
			ni.Mod(w, bdiv)
			return len(ni.Bits()) == 0
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

	for i := 0; i < 20; i++ {
		round(&ms)
		if i%10 == 0 {
			fmt.Printf("round %d\n", i)
		}
	}
	fmt.Printf("%v", ms)
}

func round(ms *[]*monkey) {
	for i := range *ms {
		turn(ms, i)
	}
}

func turn(msp *[]*monkey, i int) {
	ms := *msp
	m := ms[i]
	for len(m.items) > 0 {
		m.ins++
		it, its := m.items[0], m.items[1:]
		m.items = its
		m.op(&it)
		//w = *w.Div(&w, big.NewInt(3))
		if m.tst(&it) {
			ms[m.tr].items = append(ms[m.tr].items, it)
		} else {
			ms[m.fl].items = append(ms[m.fl].items, it)
		}
	}
}

func getOpFunc(str string) func(*big.Int) {
	exp := strings.Split(strings.Trim(str, " "), " ")
	ni, _ := strconv.Atoi(exp[len(exp)-1])
	iold := exp[len(exp)-1] != "old"

	return func(i *big.Int) {
		opnd := i
		if iold {
			opnd = big.NewInt(int64(ni))
		}
		switch exp[len(exp)-2] {
		case "+":
			i.Add(i, opnd)
		case "*":
			i.Mul(i, opnd)
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
