package main

import (
	"fmt"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}

	return a
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}

	return gcd(b, a%b)
}

func maxPoints(points [][]int) int {
	plen := len(points)
	if plen < 2 {
		return 1
	}

	result := 2

	for i := 0; i < plen; i += 1 {
		slopes := make(map[string]int)
		for j := 0; j < plen; j += 1 {
			if i == j {
				continue
			}

			pointi := points[i]
			pointj := points[j]
			x := pointj[0] - pointi[0]
			y := pointj[1] - pointi[1]
			gcd := gcd(abs(x), abs(y))
			if gcd != 0 {
				x /= gcd
				y /= gcd
			}

			key := fmt.Sprint(x, y)
			slopes[key] += 1
		}

		for _, value := range slopes {
			count := value + 1
			if count > result {
				result = count
			}
		}
	}

	return result
}

func main() {
	inputs := [][][]int{
		{{1, 1}, {2, 2}, {3, 3}},
		{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}},
	}

	for _, points := range inputs {
		result := maxPoints(points)
		fmt.Println(result)
	}
}
