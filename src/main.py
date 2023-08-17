from __future__ import annotations
from typing import List
from queue import Queue


class Solution:
    def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
        row_count = len(mat)
        col_count = len(mat[0])
        result = [[0 for _ in range(col_count)] for _ in range(row_count)]

        queue: Queue[tuple[int, int]] = Queue()
        max_value = row_count * col_count

        for r in range(row_count):
            for c in range(col_count):
                if mat[r][c] == 0:
                    queue.put((r, c))
                else:
                    result[r][c] = max_value

        directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        while not queue.empty():
            row, col = queue.get()
            cell0 = result[row][col] + 1

            for dr, dc in directions:
                r = row + dr
                c = col + dc
                if r < 0:
                    continue
                elif r >= row_count:
                    continue
                elif c < 0:
                    continue
                elif c >= col_count:
                    continue

                cell1 = result[r][c]
                if cell1 <= cell0:
                    continue

                queue.put((r, c))
                result[r][c] = cell0

        return result


def main():
    inputs = [[[0, 0, 0], [0, 1, 0], [0, 0, 0]], [[0, 0, 0], [0, 1, 0], [1, 1, 1]]]

    for mat in inputs:
        solution = Solution()
        result = solution.updateMatrix(mat)
        print(result)


if __name__ == "__main__":
    main()
