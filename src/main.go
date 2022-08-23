package main

import "fmt"

func summaryRanges(nums []int) []string {
	result := []string{}

	for i := 0; i < len(nums); i += 1 {
		head := nums[i]
		for {
			j := i + 1
			if j >= len(nums) {
				break
			}

			if (nums[i] + 1) != nums[j] {
				break
			}

			i = j
		}

		tail := nums[i]
		if head == tail {
			result = append(result, fmt.Sprint(head))
		} else {
			result = append(result, fmt.Sprintf("%d->%d", head, tail))
		}
	}

	return result
}

type input struct {
	nums []int
}

func main() {
	inputs := []input{
		{
			nums: []int{0, 1, 2, 4, 5, 7},
		},
		{
			nums: []int{0, 2, 3, 4, 6, 8, 9},
		},
	}

	for _, input := range inputs {
		nums := input.nums
		result := summaryRanges(nums)
		fmt.Println(result)
	}
}
