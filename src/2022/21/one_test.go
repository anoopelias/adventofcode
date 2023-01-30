package main

import (
	"github.com/stretchr/testify/assert"
	goutils "go-utils"
	"testing"
)

func TestOne(t *testing.T) {
	res := solve(goutils.LinesOf("input"))
	assert.Equal(t, res, "152")
}
