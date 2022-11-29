package main

import (
	"fmt"
	"math/rand"
)

type RandomizedSet struct {
	nums    []int
	indexes map[int]int
}

func Constructor() RandomizedSet {
	return RandomizedSet{nums: []int{}, indexes: map[int]int{}}
}

func (this *RandomizedSet) Insert(val int) bool {
	if _, ok := this.indexes[val]; ok {
		return false
	}

	this.indexes[val] = len(this.nums)
	this.nums = append(this.nums, val)
	return true
}

func (this *RandomizedSet) Remove(val int) bool {
	if _, ok := this.indexes[val]; !ok {
		return false
	}

	lastidx := len(this.nums) - 1
	last := this.nums[lastidx]
	pos := this.indexes[val]

	this.indexes[last] = pos
	this.nums[pos] = last

	delete(this.indexes, val)
	this.nums = this.nums[:lastidx]
	return true
}

func (this *RandomizedSet) GetRandom() int {
	idx := rand.Int() % len(this.nums)
	return this.nums[idx]
}

func main() {
	obj := Constructor()
	fmt.Println(obj.Insert(3))
	fmt.Println(obj.Insert(3))
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.Insert(1))
	fmt.Println(obj.Remove(3))
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.Insert(0))
	fmt.Println(obj.Remove(0))
}
