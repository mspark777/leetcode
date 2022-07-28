package main

func isAnagram(s string, t string) bool {
	counter := make(map[byte]int)
	for _, v := range []byte(s) {
		counter[v] += 1
	}

	for _, v := range []byte(t) {
		if count, ok := counter[v]; ok {
			if count == 1 {
				delete(counter, v)
			} else {
				counter[v] = count - 1
			}
		} else {
			return false
		}
	}

	return len(counter) < 1
}
