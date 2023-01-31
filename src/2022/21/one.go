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

type monkey struct {
	ms   *map[string]*monkey
	name string
	val  int
	set  bool
	op   int
	op1  string
	op2  string
}

func (m *monkey) calcVal() int {
	if m.set {
		//fmt.Printf("ret %s %d\n", m.name, m.val)
		return m.val
	}

	op1 := (*m.ms)[m.op1]
	op2 := (*m.ms)[m.op2]

	m.val = calc(m.op, op1.calcVal(), op2.calcVal())
	//fmt.Printf("set %s %d\n", m.name, m.val)
	m.set = true

	return m.val
}

func calc(op int, op1 int, op2 int) int {
	//fmt.Printf("calc %d %d %d\n", op, op1, op2)
	switch op {
	case plus:
		return op1 + op2
	case minus:
		return op1 - op2
	case mult:
		return op1 * op2
	case div:
		return op1 / op2
	}

	return -1
}

func solve(ls []string) string {
	ms := make(map[string]*monkey)
	for _, l := range ls {
		sps := strings.Split(l, ": ")
		nm := sps[0]
		v, err := strconv.Atoi(sps[1])
		m := &monkey{
			name: nm,
			val:  v,
			ms:   &ms,
		}

		if err == nil {
			m.set = true
			m.val = v
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

	return strconv.Itoa(ms["root"].calcVal())
}

func main1() {
	fmt.Println("Starting...")
	ls := goutils.LinesOf("input2")
	fmt.Println(solve(ls))
}
