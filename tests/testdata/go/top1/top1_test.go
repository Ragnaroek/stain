package top1

import (
	"testing"
)

func TestF1(t *testing.T) {
	if f1() != 42 {
		t.Fail()
	}
}

func TestF2(t *testing.T) {
	if f2() != 42 {
		t.Fail()
	}
}

func TestF3(t *testing.T) {
	if f3() != 42 {
		t.Fail()
	}
}
