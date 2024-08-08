from __future__ import annotations
from typing import List


class Solution:
    def spiralMatrixIII(self, rows: int, cols: int, r: int, c: int) -> List[List[int]]:
        dir = [[0, 1], [1, 0], [0, -1], [-1, 0]]
        result: list[list[int]] = []
        end = rows * cols
        step = 1
        direction = 0

        while len(result) < end:
            for _ in range(2):
                for _ in range(step):
                    is_in = r >= 0 and r < rows and c >= 0 and c < cols
                    if is_in:
                        result.append([r, c])

                    r += dir[direction][0]
                    c += dir[direction][1]

                direction = (direction + 1) % 4
            step += 1

        return result


def main():
    inputs: list[int] = [123, 12345, 1234567]

    for input in inputs:
        result = Solution().numberToWords(input)
        print(result)


if __name__ == "__main__":
    main()
