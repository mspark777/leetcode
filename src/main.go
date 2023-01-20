package main

import (
	"fmt"
)

func backtrack(nums []int, index int, sequence []int, result map[string][]int) {
	seqs := len(sequence)
	if index == len(nums) {
		if seqs >= 2 {
			key := fmt.Sprint(sequence)
			temp := make([]int, seqs)
			copy(temp, sequence)
			result[key] = temp
		}
	} else {
		num := nums[index]
		lastseq := num
		if seqs > 0 {
			lastseq = sequence[seqs-1]
		}

		if lastseq <= num {
			sequence = append(sequence, num)
			backtrack(nums, index+1, sequence, result)
			sequence = sequence[:seqs]
		}
		backtrack(nums, index+1, sequence, result)
	}
}

func findSubsequences(nums []int) [][]int {
	memo := map[string][]int{}
	backtrack(nums, 0, []int{}, memo)

	result := make([][]int, 0, len(memo))
	for _, v := range memo {
		result = append(result, v)
	}

	return result
}

func main() {
	inputs := [][]int{
		{4, 6, 7, 7},
		{4, 4, 3, 2, 1},
	}

	for _, input := range inputs {
		result := findSubsequences(input)
		fmt.Println(result)
	}
}
