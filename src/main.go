package main

import "fmt"

type node struct {
	value     string
	timestamp int
}

type TimeMap struct {
	store map[string][]node
}

func Constructor() TimeMap {
	return TimeMap{
		store: make(map[string][]node),
	}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {
	if nodes, ok := this.store[key]; ok {
		nodes = append(nodes, node{value, timestamp})
		this.store[key] = nodes
	} else {
		this.store[key] = []node{{value, timestamp}}
	}
}

func (this *TimeMap) Get(key string, timestamp int) string {
	nodes, ok := this.store[key]
	if !ok {
		return ""
	}

	if timestamp < nodes[0].timestamp {
		return ""
	}

	left := 0
	right := len(nodes)

	for left < right {
		mid := (left + right) / 2
		if nodes[mid].timestamp <= timestamp {
			left = mid + 1
		} else {
			right = mid
		}
	}

	if right < 1 {
		return ""
	}

	return nodes[right-1].value
}

func main() {
	timeMap := Constructor()
	timeMap.Set("foo", "bar", 1)
	fmt.Println(timeMap.Get("foo", 1))
	fmt.Println(timeMap.Get("foo", 3))
	timeMap.Set("foo", "bar2", 4)
	fmt.Println(timeMap.Get("foo", 4))
	fmt.Println(timeMap.Get("foo", 5))
}
