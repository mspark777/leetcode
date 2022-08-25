package main

import "fmt"

func canConstruct(ransomNote string, magazine string) bool {
	counts := make(map[rune]int)

	for _, ch := range magazine {
		counts[ch] += 1
	}

	for _, ch := range ransomNote {
		cnt := counts[ch]
		if cnt < 1 {
			return false
		}
		counts[ch] = cnt - 1
	}

	return true
}

type input struct {
	ransomNote string
	magazine   string
}

func main() {
	inputs := []input{
		{
			ransomNote: "a",
			magazine:   "b",
		},
		{
			ransomNote: "aa",
			magazine:   "ab",
		},
		{
			ransomNote: "aa",
			magazine:   "aab",
		},
	}

	for _, input := range inputs {
		ransomNote := input.ransomNote
		magazine := input.magazine
		result := canConstruct(ransomNote, magazine)
		fmt.Println(result)
	}
}
