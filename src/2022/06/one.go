package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	fmt.Println("Starting...")
	ls := linesOf("input2")

	for _, l := range ls {
		//fmt.Println(l)
		lst := []byte{}
		for j := 0; j < 14; j++ {
			lst = append(lst, l[j])
		}
		for i := 14; i < len(l); i++ {
			lst = append(lst[1:], l[i])
			if len(removeDuplicates(lst)) == len(lst) {
				fmt.Println(i + 1)
				break
			}
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

func removeDuplicates[T string | int | byte](sliceList []T) []T {
	allKeys := make(map[T]bool)
	list := []T{}
	for _, item := range sliceList {
		if _, value := allKeys[item]; !value {
			allKeys[item] = true
			list = append(list, item)
		}
	}
	return list
}
