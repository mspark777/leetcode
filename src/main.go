package main

import "fmt"

func convolute(img, kernel [][]int, xshift, yshift int) int {
	N := len(img)
	result := 0

	for r := 0; r < N; r += 1 {
		for c := 0; c < N; c += 1 {
			result += img[r][c] * kernel[r+yshift][c+xshift]
		}
	}

	return result
}

func largestOverlap(img1 [][]int, img2 [][]int) int {
	N := len(img1)
	BN := (3 * N) - 2

	bpadded := make([][]int, BN)
	for i := 0; i < BN; i += 1 {
		bpadded[i] = make([]int, BN)
	}

	for r := 0; r < N; r += 1 {
		for c := 0; c < N; c += 1 {
			bpadded[r+N-1][c+N-1] = img2[r][c]
		}
	}

	SN := (2 * N) - 1
	maxOverlaps := 0
	for xshift := 0; xshift < SN; xshift += 1 {
		for yshift := 0; yshift < SN; yshift += 1 {
			overlaps := convolute(img1, bpadded, xshift, yshift)
			if overlaps > maxOverlaps {
				maxOverlaps = overlaps
			}
		}
	}

	return maxOverlaps
}

type input struct {
	img1 [][]int
	img2 [][]int
}

func main() {
	inputs := []input{
		{
			img1: [][]int{{1, 1, 0}, {0, 1, 0}, {0, 1, 0}},
			img2: [][]int{{0, 0, 0}, {0, 1, 1}, {0, 0, 1}},
		},
		{
			img1: [][]int{{1}},
			img2: [][]int{{1}},
		},
		{
			img1: [][]int{{0}},
			img2: [][]int{{0}},
		},
		{

			img1: [][]int{{0, 0, 0}, {1, 1, 0}, {0, 0, 0}},
			img2: [][]int{{0, 1, 1}, {0, 0, 0}, {0, 0, 0}},
		},
	}

	for _, input := range inputs {
		img1 := input.img1
		img2 := input.img2
		result := largestOverlap(img1, img2)
		fmt.Println(result)
	}
}
