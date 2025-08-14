package main

import (
	"fmt"
)

type TrieNode struct {
	frequency int
	children  map[rune]*TrieNode
}

func newTrieNode() *TrieNode {
	node := new(TrieNode)
	node.frequency = 0
	node.children = make(map[rune]*TrieNode)
	return node
}

func insertWord(root *TrieNode, word string) {
	current := root
	for _, c := range word {
		if child, ok := current.children[c]; ok {
			child.frequency += 1
			current = child
		} else {
			child := newTrieNode()
			child.frequency = 1
			current.children[c] = child
			current = child
		}
	}
}

func isSubstring(root *TrieNode, word string) bool {
	current := root
	for _, c := range word {
		current = current.children[c]
	}

	return current.frequency > 1
}

func stringMatching(words []string) []string {
	result := []string{}
	root := newTrieNode()

	for _, word := range words {
		for i := 0; i < len(word); i += 1 {
			insertWord(root, word[i:])
		}
	}

	for _, word := range words {
		if isSubstring(root, word) {
			result = append(result, word)
		}
	}

	return result
}

type input struct {
	words []string
}

func main() {
	inputs := []input{
		{
			words: []string{"mass", "as", "hero", "superhero"},
		},
		{
			words: []string{"leetcode", "et", "code"},
		},
		{
			words: []string{"blue", "green", "bu"},
		},
	}

	for _, input := range inputs {
		result := stringMatching(input.words)
		fmt.Println(result)
	}
}
