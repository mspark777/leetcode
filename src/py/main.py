from __future__ import annotations
from typing import List


class Solution:
    def minDifficulty(self, job_difficulty: List[int], d: int) -> int:
        job_count = len(job_difficulty)
        if job_count < d:
            return -1
        elif job_count == d:
            return sum(job_difficulty)

        dp = [0] * job_count
        dp[0] = job_difficulty[0]

        for i in range(job_count):
            dp[i] = max(job_difficulty[i], dp[i - 1])

        dp_prev = [0] * job_count
        stack: list[int] = []

        for i in range(1, d):
            temp = dp
            dp = dp_prev
            dp_prev = temp
            stack.clear()

            for j in range(i, job_count):
                dp[j] = job_difficulty[j] + dp_prev[j - 1]

                while stack and job_difficulty[stack[-1]] <= job_difficulty[j]:
                    last_idx = stack.pop()
                    dp[j] = min(
                        dp[j],
                        dp[last_idx] + job_difficulty[j] - job_difficulty[last_idx],
                    )

                if stack:
                    dp[j] = min(dp[j], dp[stack[-1]])

                stack.append(j)

        return dp[-1]


def main():
    input = (([6, 5, 4, 3, 2, 1], 2), ([9, 9, 9], 4), ([1, 1, 1], 3))

    for jobDifficulty, d in input:
        result = Solution().minDifficulty(jobDifficulty, d)
        print(result)


if __name__ == "__main__":
    main()
