from __future__ import annotations


class Solution:
    def minimumLength(self, s: str) -> int:
        left = 0
        right = len(s) - 1
        while left < right and s[left] == s[right]:
            ch = s[left]

            while left <= right and s[left] == ch:
                left += 1

            while right > left and s[right] == ch:
                right -= 1

        return right - left + 1


def main():
    input = [
        "ca",
        "cabaabac",
        "aabccabba",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb",
    ]
    for s in input:
        result = Solution().minimumLength(s)
        print(result)


if __name__ == "__main__":
    main()
