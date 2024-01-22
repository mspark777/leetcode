from __future__ import annotations


class Solution:
    def checkRecord(self, s: str) -> bool:
        a = 0
        l = 0

        for ch in s:
            if ch == "A":
                l = 0
                a += 1
            elif ch == "L":
                l += 1
            else:
                l = 0

            if a >= 2 or l >= 3:
                return False

        return True


def main():
    input = ("PPALLP", "PPALLL", "LPLPLPLPLPL")

    for s in input:
        result = Solution().checkRecord(s)
        print(result)


if __name__ == "__main__":
    main()
