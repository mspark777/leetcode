from __future__ import annotations
from typing import Optional, List


class Solution:
    def decodeAtIndex(self, s: str, k: int) -> str:
        l = 0
        i = 0

        while l < k:
            if s[i].isdigit():
                l *= int(s[i])
            else:
                l += 1
            i += 1

        for j in range(i - 1, -1, -1):
            ch = s[j]
            if ch.isdigit():
                l //= int(ch)
                k %= l
            else:
                if (k == 0) or (k == l):
                    return ch
                l -= 1

        return ""


def main():
    inputs = [("leet2code3", 10), ("ha22", 5), ("a2345678999999999999999", 1)]

    for s, k in inputs:
        solution = Solution()
        result = solution.decodeAtIndex(s, k)
        print(result)


if __name__ == "__main__":
    main()
