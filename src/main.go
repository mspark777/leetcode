package main

import (
	"fmt"
)

func reverseBits(num uint32) uint32 {
	num = ((num & 0xFFFF0000) >> 16) | ((num & 0x0000FFFF) << 16)
	num = ((num & 0xFF00FF00) >> 8) | ((num & 0x00FF00FF) << 8)
	num = ((num & 0xF0F0F0F0) >> 4) | ((num & 0x0F0F0F0F) << 4)
	num = ((num & 0xCCCCCCCC) >> 2) | ((num & 0x33333333) << 2)
	num = ((num & 0xAAAAAAAA) >> 1) | ((num & 0x55555555) << 1)

	return num
}

type input struct {
	n uint32
}

func main() {
	inputs := []*input{
		{
			n: 43261596,
		},
		{
			n: 4294967293,
		},
	}

	for _, input := range inputs {
		n := input.n
		result := reverseBits(n)
		fmt.Println(result)
	}
}
