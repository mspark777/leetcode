from __future__ import annotations
from typing import List
from collections import deque
from heapq import heappop, heappush


class Solution:
    def maximumSafenessFactor(self, grid: List[List[int]]) -> int:
        return self.maximum_safness_factor(grid)

    def maximum_safness_factor(self, grid: list[list[int]]) -> int:
        n = len(grid)
        multi_source_queue = deque[tuple[int, int]]()
        for i in range(n):
            for j in range(n):
                if grid[i][j] == 1:
                    multi_source_queue.append((i, j))
                    grid[i][j] = 0
                else:
                    grid[i][j] = -1
        dir = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        while multi_source_queue:
            size = len(multi_source_queue)
            for _ in range(size):
                curr = multi_source_queue.popleft()
                for d in dir:
                    di, dj = curr[0] + d[0], curr[1] + d[1]
                    val = grid[curr[0]][curr[1]]
                    if self.is_valid_cell(n, di, dj) and grid[di][dj] == -1:
                        grid[di][dj] = val + 1
                        multi_source_queue.append((di, dj))

        pq = [[-grid[0][0], 0, 0]]
        grid[0][0] = -1
        dest = n - 1
        while pq:
            safeness, i, j = heappop(pq)
            if i == dest and j == dest:
                return -safeness
            for d in dir:
                di, dj = i + d[0], j + d[1]
                if self.is_valid_cell(n, di, dj) and grid[di][dj] != -1:
                    heappush(pq, [-min(-safeness, grid[di][dj]), di, dj])
                    grid[di][dj] = -1

        return -1

    def is_valid_cell(self, n: int, r: int, c: int) -> bool:
        return 0 <= r < n and 0 <= c < n


def main():
    input = [
        [[1, 0, 0], [0, 0, 0], [0, 0, 1]],
        [[0, 0, 1], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0]],
    ]

    for grid in input:
        result = Solution().maximumSafenessFactor(grid)
        print(result)


if __name__ == "__main__":
    main()
