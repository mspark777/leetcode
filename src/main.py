from __future__ import annotations
from typing import List


class Solution:
    def grayCode(self, n: int) -> List[int]:
        result: list[int] = []
        p = 1 << n
        for i in range(p):
            result.append(i ^ (i >> 1))

        return result


def main():
    inputs = [2, 1]

    for n in inputs:
        solution = Solution()
        result = solution.grayCode(n)
        print(result)


if __name__ == "__main__":
    main()
