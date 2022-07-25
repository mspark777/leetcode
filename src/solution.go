package main

func search(nums []int, target int, first bool) int {
	result := -1
	left := 0
	right := len(nums) - 1

	for left <= right {
		mid := (left + right) / 2
		num := nums[mid]

		if num > target {
			right = mid - 1
		} else if num < target {
			left = mid + 1
		} else {
			result = mid
			if first {
				right = mid - 1
			} else {
				left = mid + 1
			}
		}
	}

	return result
}

func searchRange(nums []int, target int) []int {
	first := search(nums, target, true)
	last := search(nums, target, false)

	return []int{first, last}
}
