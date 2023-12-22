from __future__ import annotations


class Solution:
    def maxScore(self, s: str) -> int:
        ones = 0
        zeros = 0
        score = -1000
        s_len = len(s)

        for ch in s[: s_len - 1]:
            if ch == "1":
                ones += 1
            else:
                zeros += 1

            score = max(score, zeros - ones)

        if s[-1] == "1":
            ones += 1

        return score + ones


def main():
    input = ("011101", "00111", "1111")

    for s in input:
        result = Solution().maxScore(s)
        print(result)


if __name__ == "__main__":
    main()
