package main

const LETTER_COUNT = 26
const ACODE = 'a'

func getCounts(word string) []int {
	counts := make([]int, LETTER_COUNT)
	for _, ch := range word {
		i := ch - ACODE
		counts[i] += 1
	}

	return counts
}

func wordSubsets(words1 []string, words2 []string) []string {
	counts2 := getCounts("")
	for _, word := range words2 {
		counts3 := getCounts(word)
		for i := 0; i < LETTER_COUNT; i += 1 {
			c2 := counts2[i]
			c3 := counts3[i]
			if c2 < c3 {
				counts2[i] = c3
			}
		}
	}

	result := []string{}
	for _, word := range words1 {
		counts1 := getCounts(word)
		ok := true
		for i := 0; i < LETTER_COUNT; i += 1 {
			if counts1[i] < counts2[i] {
				ok = false
				break
			}
		}

		if ok {
			result = append(result, word)
		}
	}

	return result
}
