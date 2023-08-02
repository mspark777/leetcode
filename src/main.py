from __future__ import annotations


class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        si = 0
        pi = 0
        mi = 0
        stari = -1

        while si < len(s):
            if (pi < len(p)) and ((p[pi] == "?") or (s[si] == p[pi])):
                si += 1
                pi += 1
            elif (pi < len(p)) and (p[pi] == "*"):
                stari = pi
                mi = si
                pi += 1
            elif stari >= 0:
                pi = stari
                mi += 1
                si = mi
            else:
                return False

        while pi < len(p):
            if p[pi] == "*":
                pi += 1
            else:
                break

        return pi == len(p)


def main():
    inputs = [("aa", "a"), ("aa", "*"), ("cb", "?a")]

    for s, p in inputs:
        solution = Solution()
        result = solution.isMatch(s, p)
        print(result)


if __name__ == "__main__":
    main()
