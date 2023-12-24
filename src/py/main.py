from __future__ import annotations


class Solution:
    def minOperations(self, s: str) -> int:
        start0 = 0

        for i, ch in enumerate(s):
            if i % 2 == 0:
                if ch == "1":
                    start0 += 1
            elif ch == "0":
                start0 += 1

        return min(start0, len(s) - start0)


def main():
    input = ("0100", "10", "1111")

    for s in input:
        result = Solution().minOperations(s)
        print(result)


if __name__ == "__main__":
    main()
