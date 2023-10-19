from __future__ import annotations
from typing import Optional, List


class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        slist = list(s)
        tlist = list(t)

        si = len(slist) - 1
        ti = len(tlist) - 1

        while si >= 0 and ti >= 0:
            si = self.next(slist, si)
            ti = self.next(tlist, ti)

            if si < 0 or ti < 0:
                break

            if slist[si] != tlist[ti]:
                return False

            si -= 1
            ti -= 1

        if si >= 0:
            si = self.next(slist, si)

        if ti >= 0:
            ti = self.next(tlist, ti)

        return si < 0 and ti < 0

    def next(self, s: list[str], i: int) -> int:
        skip = 0
        while i >= 0:
            if s[i] == "#":
                skip += 1
                i -= 1
            elif skip > 0:
                skip -= 1
                i -= 1
            else:
                break

        return i


def main():
    inputs = (("ab#c", "ad#c"), ("ab##", "c#d#"), ("a#c", "b"))

    for s, t in inputs:
        result = Solution().backspaceCompare(s, t)
        print(result)


if __name__ == "__main__":
    main()
