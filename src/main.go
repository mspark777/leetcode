package main

import (
	"fmt"
)

type MyHashSet struct {
	nums []bool
}

func Constructor() MyHashSet {
	return MyHashSet{
		nums: make([]bool, 1000001),
	}
}

func (this *MyHashSet) Add(key int) {
	this.nums[key] = true
}

func (this *MyHashSet) Remove(key int) {
	this.nums[key] = false
}

func (this *MyHashSet) Contains(key int) bool {
	return this.nums[key]
}

func main() {
	myHashSet := Constructor()
	myHashSet.Add(1)
	myHashSet.Add(2)
	fmt.Println(myHashSet.Contains(1))
	fmt.Println(myHashSet.Contains(3))
	myHashSet.Add(2)
	fmt.Println(myHashSet.Contains(2))
	myHashSet.Remove(2)
	fmt.Println(myHashSet.Contains(2))
}
