package main

import (
	"fmt"
	"sort"
)

type KthLargest struct {
	k    int
	nums []int
}

func Constructor(k int, nums []int) KthLargest {
	sort.Slice(nums, func(i, j int) bool {
		return nums[j] < nums[i]
	})

	return KthLargest{
		k:    k - 1,
		nums: nums,
	}
}

func (this *KthLargest) Add(val int) int {
	i := 0
	for i < len(this.nums) {
		if val > this.nums[i] {
			break
		} else {
			i += 1
		}
	}

	this.nums = append(this.nums, val)
	if i < len(this.nums) {
		for j := len(this.nums) - 1; j > i; j -= 1 {
			this.nums[j] = this.nums[j-1]
		}
		this.nums[i] = val
	}

	return this.nums[this.k]
}

func main() {
	kthLargest := Constructor(3, []int{4, 5, 8, 1})
	fmt.Println(kthLargest.Add(3))
	fmt.Println(kthLargest.Add(5))
	fmt.Println(kthLargest.Add(10))
	fmt.Println(kthLargest.Add(9))
	fmt.Println(kthLargest.Add(4))
}
