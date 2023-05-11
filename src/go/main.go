package main

import (
	"fmt"
)

func maxUncrossedLines(nums1 []int, nums2 []int) int {
	len1 := len(nums1)
	len2 := len(nums2)

	dp := make([]int, len2+1)
	dpPrev := make([]int, len2+1)

	for i := 1; i <= len1; i += 1 {
		for j := 1; j <= len2; j += 1 {
			if nums1[i-1] == nums2[j-1] {
				dp[j] = 1 + dpPrev[j-1]
			} else {
				cur := dp[j-1]
				prev := dpPrev[j]
				if cur < prev {
					dp[j] = prev
				} else {
					dp[j] = cur
				}
			}
		}

		copy(dpPrev, dp)
	}

	return dp[len2]
}

func main() {
	inputs := [][][]int{
		{{1, 4, 2}, {1, 2, 4}},
		{{2, 5, 1, 2, 5}, {10, 5, 2, 1, 5, 2}},
		{{1, 3, 7, 1, 7, 5}, {1, 9, 2, 5, 1}},
		{{3, 2}, {2, 2, 2, 3}},
	}

	for _, input := range inputs {
		result := maxUncrossedLines(input[0], input[1])
		fmt.Println(result)
	}
}
