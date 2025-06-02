package main

import "fmt"

func count(n int) int {
	return (n * (n + 1)) / 2
}

func candy(ratings []int) int {
	ratings_len := len(ratings)
	if ratings_len < 2 {
		return ratings_len
	}

	candies := 0
	up := 0
	down := 0
	old_slope := 0

	for i := 1; i < ratings_len; i += 1 {
		cur := ratings[i]
		prev := ratings[i-1]
		slope := 0
		if cur > prev {
			slope = 1
		} else if cur < prev {
			slope = -1
		}

		if ((old_slope > 0) && (slope == 0)) || ((old_slope < 0) && (slope >= 0)) {
			candies += count(up) + count(down) + max(up, down)
			up = 0
			down = 0
		}

		if slope > 0 {
			up += 1
		} else if slope < 0 {
			down += 1
		} else {
			candies += 1
		}

		old_slope = slope
	}

	candies += count(up) + count(down) + max(up, down) + 1
	return candies
}

type input struct {
	ratings []int
}

func main() {
	inputs := []input{
		{
			ratings: []int{1, 0, 2},
		},
		{
			ratings: []int{1, 2, 2},
		},
	}

	for _, input := range inputs {
		result := candy(input.ratings)
		fmt.Println(result)
	}
}
