package main

import (
	"fmt"
)

type SmallestInfiniteSet struct {
	current int
	memo    map[int]bool
}

func Constructor() SmallestInfiniteSet {
	return SmallestInfiniteSet{
		current: 1,
		memo:    map[int]bool{},
	}
}

func (this *SmallestInfiniteSet) PopSmallest() int {
	if len(this.memo) < 1 {
		result := this.current
		this.current += 1
		return result
	}

	result := int((^uint(0)) >> 1)
	for num := range this.memo {
		if num < result {
			result = num
		}
	}

	delete(this.memo, result)
	return result
}

func (this *SmallestInfiniteSet) AddBack(num int) {
	if this.current <= num {
		return
	} else if _, ok := this.memo[num]; ok {
		return
	}

	this.memo[num] = true
}

func main() {
	smallestInfiniteSet := Constructor()
	smallestInfiniteSet.AddBack(2)
	fmt.Println(smallestInfiniteSet.PopSmallest())
	fmt.Println(smallestInfiniteSet.PopSmallest())
	fmt.Println(smallestInfiniteSet.PopSmallest())
	smallestInfiniteSet.AddBack(1)
	fmt.Println(smallestInfiniteSet.PopSmallest())
	fmt.Println(smallestInfiniteSet.PopSmallest())
	fmt.Println(smallestInfiniteSet.PopSmallest())
}
