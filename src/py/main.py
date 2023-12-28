from __future__ import annotations


class Solution:
    def getLengthOfOptimalCompression(self, s: str, k: int) -> int:
        n = len(s)
        m = k

        dp = [[0] * 110 for _ in range(110)]

        for i in range(1, n + 1):
            for j in range(min(n, m) + 1):
                need_remove = 0
                group_count = 0
                dp[i][j] = 2**31
                if j > 0:
                    dp[i][j] = dp[i - 1][j - 1]

                for k in range(i, 0, -1):
                    if s[k - 1] != s[i - 1]:
                        need_remove += 1
                    else:
                        group_count += 1

                    if need_remove > j:
                        break

                    if group_count == 1:
                        dp[i][j] = min(dp[i][j], dp[k - 1][j - need_remove] + 1)
                    elif group_count < 10:
                        dp[i][j] = min(dp[i][j], dp[k - 1][j - need_remove] + 2)
                    elif group_count < 100:
                        dp[i][j] = min(dp[i][j], dp[k - 1][j - need_remove] + 3)
                    else:
                        dp[i][j] = min(dp[i][j], dp[k - 1][j - need_remove] + 4)

        return dp[n][m]


def main():
    input = (("aaabcccd", 2), ("aabbaa", 2), ("aaaaaaaaaaa", 0))

    for s, k in input:
        result = Solution().getLengthOfOptimalCompression(s, k)
        print(result)


if __name__ == "__main__":
    main()
