package main

func findPattern(word string, pattern string) bool {
	wrunes := []rune(word)
	prunes := []rune(pattern)
	if len(wrunes) != len(prunes) {
		return false
	}

	wmap := make(map[rune]rune)
	pmap := make(map[rune]rune)

	for i, wc := range wrunes {
		pc := prunes[i]

		if _, ok := wmap[wc]; !ok {
			wmap[wc] = pc
		}

		if _, ok := pmap[pc]; !ok {
			pmap[pc] = wc
		}

		if wmap[wc] != pc {
			return false
		}

		if pmap[pc] != wc {
			return false
		}
	}

	return true
}

func findAndReplacePattern(words []string, pattern string) []string {
	result := []string{}
	for _, word := range words {
		if findPattern(word, pattern) {
			result = append(result, word)
		}
	}

	return result
}
