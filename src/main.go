package main

import (
	"fmt"
	"sort"
)

type heapNode struct {
	word  string
	count int
}

func topKFrequent(words []string, k int) []string {
	counts := map[string]int{}
	for _, word := range words {
		counts[word] += 1
	}

	heap := []heapNode{}
	for word, count := range counts {
		heap = append(heap, heapNode{word, count})
	}

	sort.Slice(heap, func(i, j int) bool {
		a := &heap[i]
		b := &heap[j]

		if a.count != b.count {
			return a.count > b.count
		}

		return a.word < b.word
	})

	result := []string{}
	for _, node := range heap {
		result = append(result, node.word)
	}

	return result[0:k]
}

type input struct {
	words []string
	k     int
}

func main() {
	inputs := []input{
		{
			words: []string{"i", "love", "leetcode", "i", "love", "coding"},
			k:     2,
		},
		{
			words: []string{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"},
			k:     4,
		},
	}

	for _, input := range inputs {
		words := input.words
		k := input.k
		result := topKFrequent(words, k)
		fmt.Println(result)
	}
}
