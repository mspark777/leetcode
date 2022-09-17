package main

import (
	"fmt"
)

type trieNode struct {
	children []*trieNode
	index    int
	indices  []int
}

func newnode() *trieNode {
	return &trieNode{
		children: make([]*trieNode, 26),
		index:    -1,
		indices:  []int{},
	}
}

func isPalindrome(s []byte, head, tail int) bool {
	for head < tail {
		if s[head] != s[tail] {
			return false
		}

		head += 1
		tail -= 1
	}

	return true
}

func addNode(node *trieNode, word []byte, index int) {
	for i := len(word) - 1; i >= 0; i -= 1 {
		ch := word[i] - byte('a')
		if node.children[ch] == nil {
			node.children[ch] = newnode()
		}

		if isPalindrome(word, 0, i) {
			node.indices = append(node.indices, index)
		}

		node = node.children[ch]
	}

	node.indices = append(node.indices, index)
	node.index = index
}

func searchNode(result [][]int, word []byte, index int, node *trieNode) [][]int {
	for i := 0; i < len(word); i += 1 {
		if node == nil {
			return result
		}

		if (node.index >= 0) && (node.index != index) && isPalindrome(word, i, len(word)-1) {
			result = append(result, []int{index, node.index})
		}

		ch := word[i] - byte('a')
		node = node.children[ch]
	}

	if node == nil {
		return result
	}

	for _, j := range node.indices {
		if index != j {
			result = append(result, []int{index, j})
		}
	}

	return result
}

func palindromePairs(words []string) [][]int {
	root := newnode()
	for i, word := range words {
		addNode(root, []byte(word), i)
	}

	result := [][]int{}
	for i, word := range words {
		result = searchNode(result, []byte(word), i, root)
	}

	return result
}

type input struct {
	words []string
}

func main() {
	inputs := []input{
		{
			words: []string{"abcd", "dcba", "lls", "s", "sssll"},
		},
		{
			words: []string{"bat", "tab", "cat"},
		},
		{
			words: []string{"a", ""},
		},
	}

	for _, input := range inputs {
		words := input.words
		result := palindromePairs(words)
		fmt.Println(result)
	}
}
