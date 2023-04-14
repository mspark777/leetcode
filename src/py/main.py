class Solution:
    def longestPalindromeSubseq(self, s: str) -> int:
        n = len(s)
        dp = [0] * n
        dp_prev = dp.copy()

        for i in range(n - 1, -1, -1):
            dp[i] = 1
            for j in range(i + 1, n):
                if s[i] == s[j]:
                    dp[j] = dp_prev[j - 1] + 2
                else:
                    dp[j] = max(dp_prev[j], dp[j - 1])
            dp_prev = dp.copy()

        return dp[n - 1]


def main():
    inputs: list[str] = ["bbbab", "cbbd"]

    for s in inputs:
        solution = Solution()
        result = solution.longestPalindromeSubseq(s)
        print(result)


if __name__ == "__main__":
    main()
