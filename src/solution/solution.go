package solution

func KInversePairs(n int, k int) int {
	return kInversePairs(n, k)
}

func kInversePairs(n int, k int) int {
	dp := make([]int, k+1)
	const MODULO = 1000000007

	for i := 1; i <= n; i += 1 {
		temp := make([]int, k+1)
		temp[0] = 1
		for j := 1; j <= k; j += 1 {
			v := dp[j] + MODULO
			if j >= i {
				v -= dp[j-i]
			}

			v %= MODULO
			temp[j] = (temp[j-1] + v) % MODULO
		}

		dp = temp
	}

	result := dp[k] + MODULO
	if k > 0 {
		result -= dp[k-1]
	}

	return result % MODULO
}
