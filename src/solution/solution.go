package solution

func FindPaths(m int, n int, maxMove int, startRow int, startColumn int) int {
	return findPaths(m, n, maxMove, startRow, startColumn)
}

func findPaths(m int, n int, maxMove int, startRow int, startColumn int) int {
	const MODULO = 1000000007
	createDP := func() [][]int {
		dp := make([][]int, m)
		for i := range dp {
			dp[i] = make([]int, n)
		}
		return dp
	}

	dp := createDP()
	dp[startRow][startColumn] = 1
	count := 0

	for moves := 0; moves < maxMove; moves += 1 {
		temp := createDP()
		for i := 0; i < m; i += 1 {
			for j := 0; j < n; j += 1 {
				if i == (m - 1) {
					count = (count + dp[i][j]) % MODULO
				}

				if j == (n - 1) {
					count = (count + dp[i][j]) % MODULO
				}

				if i == 0 {
					count = (count + dp[i][j]) % MODULO
				}

				if j == 0 {
					count = (count + dp[i][j]) % MODULO
				}

				ti := 0
				if i > 0 {
					ti += dp[i-1][j]
				}

				if i < (m - 1) {
					ti += dp[i+1][j]
				}
				ti %= MODULO

				tj := 0
				if j > 0 {
					tj += dp[i][j-1]
				}

				if j < (n - 1) {
					tj += dp[i][j+1]
				}
				tj %= MODULO

				temp[i][j] = (ti + tj) % MODULO
			}
		}
		dp = temp
	}

	return count
}
