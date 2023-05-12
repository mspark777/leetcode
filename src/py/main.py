from typing import List


class Solution:
    def mostPoints(self, questions: List[List[int]]) -> int:
        qlen = len(questions)
        last = qlen - 1
        dp = [0 for i in range(qlen)]
        dp[last] = questions[last][0]

        for i in range(last - 1, -1, -1):
            [point, power] = questions[i]
            dp[i] = point

            if (i + power) < last:
                dp[i] += dp[i + power + 1]

            dp[i] = max(dp[i], dp[i + 1])

        return dp[0]


def main():
    inputs = [
        [[3, 2], [4, 3], [4, 4], [2, 5]],
        [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]],
    ]

    for questions in inputs:
        solution = Solution()
        result = solution.mostPoints(questions)
        print(result)


if __name__ == "__main__":
    main()
