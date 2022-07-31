package main

func buildTree(nums []int) []int {
	length := len(nums)
	tree := make([]int, length*2)
	for i := 0; i < length; i += 1 {
		tree[i+length] = nums[i]
	}

	for i := length - 1; i > 0; i -= 1 {
		tree[i] = tree[i*2] + tree[i*2+1]
	}

	return tree
}

type NumArray struct {
	tree     []int
	numCount int
}

func Constructor(nums []int) NumArray {
	return NumArray{
		tree:     buildTree(nums),
		numCount: len(nums),
	}
}

func (this *NumArray) Update(index int, val int) {
	tree := this.tree
	index += this.numCount

	tree[index] = val
	for index > 0 {
		left := index
		right := index
		if index%2 == 0 {
			right = index + 1
		} else {
			left = index - 1
		}

		index /= 2
		tree[index] = tree[left] + tree[right]
	}
}

func (this *NumArray) SumRange(left int, right int) int {
	tree := this.tree
	length := this.numCount

	left += length
	right += length

	sum := 0
	for left <= right {
		if left%2 == 1 {
			sum += tree[left]
			left += 1
		}

		if right%2 == 0 {
			sum += tree[right]
			right -= 1
		}

		left /= 2
		right /= 2
	}

	return sum
}
