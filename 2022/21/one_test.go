package main

import (
	"github.com/stretchr/testify/assert"
	goutils "go-utils"
	"testing"
)

func TestOne(t *testing.T) {
	res := solve(goutils.LinesOf("../../../aoc-files/2022/21/input"))
	assert.Equal(t, res, "152")
}
