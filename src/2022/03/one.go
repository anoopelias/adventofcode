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

	for _, l := range ls {
		lt := l[:len(l)/2]
		rt := l[len(l)/2:]

		for _, c := range lt {
			if strings.Contains(rt, string(c)) {
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
