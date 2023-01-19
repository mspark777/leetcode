package main

import (
	"fmt"
	"math/bits"
)

func readBinaryWatch(turnedOn int) []string {
	result := []string{}

	for h := 0; h < 12; h += 1 {
		for m := 0; m < 60; m += 1 {
			num := (h << 6) | m
			ones := bits.OnesCount(uint(num))
			if ones == turnedOn {
				if m >= 10 {
					result = append(result, fmt.Sprintf("%v:%v", h, m))
				} else {
					result = append(result, fmt.Sprintf("%v:0%v", h, m))
				}
			}
		}
	}

	return result
}

func main() {
	inputs := []int{1, 9}

	for _, input := range inputs {
		result := readBinaryWatch(input)
		fmt.Println(result)
	}
}
