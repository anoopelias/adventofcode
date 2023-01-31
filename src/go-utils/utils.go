package goutils

import (
	"io/ioutil"
	"strings"
)

func LinesOf(fn string) []string {
	fbyts, err := ioutil.ReadFile(fn)
	if err != nil {
		panic(err)
	}
	lines := make([]string, 0)
	lines = append(lines, strings.Split(string(fbyts), "\n")...)
	return lines
}
