from __future__ import annotations
from queue import PriorityQueue


class Solution:
    def minimumEffortPath(self, heights: list[list[int]]) -> int:
        efforts = [[2**31 for _ in row] for row in heights]
        efforts[0][0] = 0

        return self.dijkstra(heights, efforts)

    def dijkstra(self, heights: list[list[int]], efforts: list[list[int]]) -> int:
        row_count = len(heights)
        col_count = len(heights[0])

        queue = PriorityQueue[tuple[int, tuple[int, int]]]()
        queue.put((0, (0, 0)))

        dirs: list[tuple[int, int]] = [(0, 1), (1, 0), (-1, 0), (0, -1)]

        while not queue.empty():
            (cost, (r, c)) = queue.get()

            if cost > efforts[r][c]:
                continue

            if ((r + 1) == row_count) and ((c + 1) == col_count):
                return cost

            for dr, dc in dirs:
                new_r = r + dr
                new_c = c + dc

                if (new_r < 0) or (new_c < 0):
                    continue
                elif (new_r >= row_count) or (new_c >= col_count):
                    continue

                new_effort = max(
                    efforts[r][c], abs(heights[r][c] - heights[new_r][new_c])
                )

                if new_effort < efforts[new_r][new_c]:
                    efforts[new_r][new_c] = new_effort
                    queue.put((new_effort, (new_r, new_c)))

        return efforts[-1][-1]


def main():
    inputs = [
        [[1, 2, 2], [3, 8, 2], [5, 3, 5]],
        [[1, 2, 3], [3, 8, 4], [5, 3, 5]],
        [
            [1, 2, 1, 1, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 1, 1, 2, 1],
        ],
    ]

    for heights in inputs:
        solution = Solution()
        result = solution.minimumEffortPath(heights)
        print(result)


if __name__ == "__main__":
    main()
