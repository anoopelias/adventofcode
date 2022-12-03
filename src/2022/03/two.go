package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	"unicode"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input")
	p := 0

	for i := 0; i < len(ls)/3; i++ {
		n := i * 3
		f := ls[n]
		s := ls[n+1]
		t := ls[n+2]

		for _, c := range f {
			if strings.Contains(s, string(c)) && strings.Contains(t, string(c)) {
				p += valueOf(c)
				break
			}
		}
	}

	fmt.Println(p)
}

func valueOf(c rune) int {
	if unicode.IsUpper(c) {
		return int(c) - int('A') + 27
	}

	return int(c) - int('a') + 1
}

func linesOf(fn string) []string {
	fbyts, err := ioutil.ReadFile(fn)
	if err != nil {
		panic(err)
	}
	lines := make([]string, 3)
	for _, line := range strings.Split(string(fbyts), "\n") {
		lines = append(lines, line)

	}

	return lines
}
