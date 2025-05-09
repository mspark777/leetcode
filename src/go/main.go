package main

import "fmt"

func countBalancedPermutations(num string) int {
	const MOD = 1_000_000_007
	tot := 0
	n := len(num)
	cnt := make([]int, 10)
	for _, ch := range num {
		d := int(ch - '0')
		cnt[d] += 1
		tot += d
	}

	if (tot & 1) == 1 {
		return 0
	}

	target := tot / 2
	maxOdd := (n + 1) / 2
	comb := make([][]int, maxOdd+1)

	for i := range comb {
		comb[i] = make([]int, maxOdd+1)
		comb[i][i] = 1
		comb[i][0] = 1
		for j := 1; j < i; j += 1 {
			comb[i][j] = (comb[i-1][j] + comb[i-1][j-1]) % MOD
		}
	}
	f := make([][]int, target+1)
	for i := range f {
		f[i] = make([]int, maxOdd+1)
	}
	f[0][0] = 1
	psum, totSum := 0, 0
	for i := 0; i <= 9; i += 1 {
		psum += cnt[i]
		totSum += i * cnt[i]
		for oddCnt := min(psum, maxOdd); oddCnt >= max(0, psum-(n-maxOdd)); oddCnt -= 1 {
			evenCnt := psum - oddCnt
			for curr := min(totSum, target); curr >= max(0, totSum-target); curr -= 1 {
				res := 0
				for j := max(0, cnt[i]-evenCnt); j <= min(cnt[i], oddCnt) && i*j <= curr; j += 1 {
					ways := comb[oddCnt][j] * comb[evenCnt][cnt[i]-j] % MOD
					res = (res + ways*f[curr-i*j][oddCnt-j]%MOD) % MOD
				}
				f[curr][oddCnt] = res % MOD
			}
		}
	}
	return f[target][maxOdd]
}

type input struct {
	num string
}

func main() {
	inputs := []input{
		{
			num: "123",
		},
		{
			num: "112",
		},
		{
			num: "12345",
		},
	}

	for _, input := range inputs {
		result := countBalancedPermutations(input.num)
		fmt.Println(result)
	}
}
