package solution

func NumSubmatrixSumTarget(matrix [][]int, target int) int {
	return numSubmatrixSumTarget(matrix, target)
}

func numSubmatrixSumTarget(matrix [][]int, target int) int {
	result := 0
	m := len(matrix)
	n := len(matrix[0])

	for i := 0; i < m; i += 1 {
		for j := 1; j < n; j += 1 {
			matrix[i][j] += matrix[i][j-1]
		}
	}

	counter := make(map[int]int, n)
	for i := 0; i < n; i += 1 {
		for j := i; j < n; j += 1 {
			for k := range counter {
				delete(counter, k)
			}
			counter[0] = 1

			cur := 0
			for k := 0; k < m; k += 1 {
				cur += matrix[k][j]
				if i > 0 {
					cur -= matrix[k][i-1]
				}

				if c, ok := counter[cur-target]; ok {
					result += c
				}

				if c, ok := counter[cur]; ok {
					counter[cur] = c + 1
				} else {
					counter[cur] = 1
				}
			}
		}
	}

	return result
}
