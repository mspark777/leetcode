from __future__ import annotations
from typing import List


class Solution:
    def imageSmoother(self, img: List[List[int]]) -> List[List[int]]:
        row_count = len(img)
        col_count = len(img[0])
        result = [[0] * col_count for _ in range(row_count)]

        for r in range(row_count):
            for c in range(col_count):
                result[r][c] = self.average(img, r, c)

        return result

    def average(self, img: list[list[int]], row: int, col: int) -> int:
        row_count = len(img)
        col_count = len(img[0])
        total = 0
        cell_count = 0
        offsets = [(r, c) for c in range(-1, 2) for r in range(-1, 2)]

        for offset_r, offset_c in offsets:
            r = row + offset_r
            c = col + offset_c

            if r < 0 or c < 0:
                continue
            elif r >= row_count or c >= col_count:
                continue

            total += img[r][c]
            cell_count += 1

        return total // cell_count


def main():
    inputs = (
        [[1, 1, 1], [1, 0, 1], [1, 1, 1]],
        [[100, 200, 100], [200, 50, 200], [100, 200, 100]],
    )

    for img in inputs:
        result = Solution().imageSmoother(img)
        print(result)


if __name__ == "__main__":
    main()
