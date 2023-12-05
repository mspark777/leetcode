from __future__ import annotations


class Solution:
    def numberOfMatches(self, n: int) -> int:
        return n - 1


def main():
    inputs = (7, 14)

    for n in inputs:
        result = Solution().numberOfMatches(n)
        print(result)


if __name__ == "__main__":
    main()
