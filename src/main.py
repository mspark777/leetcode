from __future__ import annotations
from typing import List


class Solution:
    def countBits(self, n: int) -> List[int]:
        result = [0 for _ in range(n + 1)]
        for i in range(1, n + 1):
            result[i] = result[i >> 1] + (i & 1)

        return result


def main():
    inputs = [2, 5]

    for n in inputs:
        solution = Solution()
        result = solution.countBits(n)
        print(result)


if __name__ == "__main__":
    main()
