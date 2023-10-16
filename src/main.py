from __future__ import annotations
from typing import Optional, List


class Solution:
    def getRow(self, row_index: int) -> List[int]:
        result = [1] * (row_index + 1)

        for i in range(len(result)):
            for j in range(i - 1, 0, -1):
                result[j] += result[j - 1]

        return result


def main():
    inputs = (3, 0, 1)

    for row in inputs:
        result = Solution().getRow(row)
        print(result)


if __name__ == "__main__":
    main()
