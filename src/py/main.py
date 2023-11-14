from __future__ import annotations


class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        first = [-1] * 26
        last = [-1] * 26

        for i, c in enumerate(s):
            curr = ord(c) - ord("a")
            if first[curr] == -1:
                first[curr] = i

            last[curr] = i

        result = 0
        for i in range(26):
            if first[i] == -1:
                continue

            between = set[str]()
            for j in range(first[i] + 1, last[i]):
                between.add(s[j])

            result += len(between)

        return result


def main():
    inputs = ("aabca", "adc", "bbcbaba")

    for s in inputs:
        result = Solution().countPalindromicSubsequence(s)
        print(result)


if __name__ == "__main__":
    main()
