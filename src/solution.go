package main

func numMatchingSubseq(s string, words []string) int {
	seen := make(map[string]bool)
	result := 0

	for _, word := range words {
		if check, ok := seen[word]; ok {
			if check {
				result += 1
			}
			continue
		}

		matched := 0
		for i, j := 0, 0; i < len(s) && j < len(word); i += 1 {
			if s[i] == word[j] {
				matched += 1
				j += 1
			}
		}

		if matched == len(word) {
			result += 1
			seen[word] = true
		} else {
			seen[word] = false
		}
	}

	return result
}
