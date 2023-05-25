class Solution:
    def new21Game(self, n: int, k: int, max_pts: int) -> float:
        if k == 0:
            return 1.0
        elif n >= (k + max_pts):
            return 1.0

        dp = [0.0 for _ in range(n + 1)]
        dp[0] = 1.0
        sum = 1.0
        result = 0.0

        for i in range(1, n + 1):
            dp[i] = sum / max_pts
            if i < k:
                sum += dp[i]
            else:
                result += dp[i]

            if (i - max_pts) >= 0:
                sum -= dp[i - max_pts]

        return result


def main():
    inputs = [(10, 1, 10), (6, 1, 10), (21, 17, 10)]

    for n, k, max_pts in inputs:
        solution = Solution()
        result = solution.new21Game(n, k, max_pts)
        print(result)


if __name__ == "__main__":
    main()
