package main

import (
	"fmt"
)

func validUtf8(data []int) bool {
	bytes := 0

	for _, i := range data {
		if bytes == 0 {
			for mask := 128; (mask & i) != 0; mask >>= 1 {
				bytes += 1
			}

			if bytes == 0 {
				continue
			}

			if (bytes > 4) || (bytes == 1) {
				return false
			}
		} else {
			if ((i & 128) == 0) || ((i & 64) != 0) {
				return false
			}
		}

		bytes -= 1
	}

	return bytes == 0
}

type input struct {
	data []int
}

func main() {
	inputs := []input{
		{
			data: []int{197, 130, 1},
		},
		{
			data: []int{235, 140, 4},
		},
		{
			data: []int{240, 162, 138, 147},
		},
	}

	for _, input := range inputs {
		data := input.data
		result := validUtf8(data)
		fmt.Println(result)
	}
}
