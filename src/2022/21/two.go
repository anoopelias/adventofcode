package main

import (
	"fmt"
	goutils "go-utils"
	"strconv"
	"strings"
)

const (
	plus int = iota
	minus
	mult
	div
)

type eq struct {
	m float64
	c float64
}

type monkey struct {
	ms   *map[string]*monkey
	name string
	val  eq
	set  bool
	op   int
	op1  string
	op2  string
}

func (m *monkey) calcVal() eq {
	if m.set {
		return m.val
	}

	op1 := (*m.ms)[m.op1]
	op2 := (*m.ms)[m.op2]

	m.val = calc(m.op, op1.calcVal(), op2.calcVal())
	m.set = true

	return m.val
}

func calc(op int, op1 eq, op2 eq) eq {

	if op == plus {
		return eq{
			op1.m + op2.m,
			op1.c + op2.c,
		}
	}

	if op == minus {
		return eq{
			op1.m - op2.m,
			op1.c - op2.c,
		}
	}

	if op == mult {
		if op1.m == 0 || op2.m == 0 {
			if op1.m == 0 {
				op1, op2 = op2, op1
			}
			// op2.m is 0
			return eq{
				op1.m * op2.c,
				op1.c * op2.c,
			}
		}
		fmt.Printf("Quadratic equation\n")
		return eq{}
	}
	if op == div {
		if op2.m == 0 {
			return eq{
				op1.m / op2.c,
				op1.c / op2.c,
			}
		}
		fmt.Printf("1/x equation\n")
		return eq{}
	}

	fmt.Printf("invalid operation %d\n", op)

	return eq{}
}

func solve(ls []string) string {
	ms := make(map[string]*monkey)
	for _, l := range ls {
		sps := strings.Split(l, ": ")
		nm := sps[0]
		v, err := strconv.Atoi(sps[1])
		val := eq{c: float64(v)}
		m := &monkey{
			name: nm,
			ms:   &ms,
		}

		if err == nil {
			m.set = true
			m.val = val
		} else {
			sps = strings.Split(sps[1], " ")
			m.op1 = sps[0]
			m.op2 = sps[2]
			switch sps[1] {
			case "+":
				m.op = plus
			case "-":
				m.op = minus
			case "*":
				m.op = mult
			case "/":
				m.op = div
			}
		}
		ms[nm] = m
	}

	humn := ms["humn"]
	humn.val = eq{1, 0}
	humn.set = true

	root := ms["root"]
	eq1 := ms[root.op1].calcVal()
	eq2 := ms[root.op2].calcVal()

	if eq2.m == 0 {
		return strconv.Itoa(int((eq2.c - eq1.c) / eq1.m))
	}

	return strconv.Itoa(int((eq1.c - eq2.c) / eq2.m))
}

func main() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("input2")
	fmt.Println(solve(ls))
}
