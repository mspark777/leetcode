from __future__ import annotations
from typing import Optional, List


class Solution:
    def longestPalindrome(self, s: str) -> str:
        if len(s) < 2:
            return s
        elif len(s) == 2:
            return s if s[0] == s[1] else s[0]

        slen = len(s)
        maxlen = 0
        start = 0
        for i in range(1, slen - 1):
            ch = s[i]
            low = i - 1

            while low > -1:
                if s[low] == ch:
                    low -= 1
                else:
                    break

            high = i + 1
            while high < slen:
                if s[high] == ch:
                    high += 1
                else:
                    break

            while low > -1 and high < slen:
                if s[low] == s[high]:
                    low -= 1
                    high += 1
                else:
                    break

            l = high - low - 1
            if maxlen < l:
                maxlen = l
                start = low + 1

        return s[start : maxlen + start]


def main():
    inputs = ("babad", "cbbd")

    for s in inputs:
        result = Solution().longestPalindrome(s)
        print(result)


if __name__ == "__main__":
    main()
