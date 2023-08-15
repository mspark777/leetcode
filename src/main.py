from __future__ import annotations
from typing import List
from collections import deque, defaultdict


class Solution:
    def shift_for_all_keys(self, cell: str, stand: str) -> int:
        return 1 << (ord(cell) - ord(stand))

    def shortestPathAllKeys(self, grid: List[str]) -> int:
        row_count = len(grid)
        col_count = len(grid[0])
        queue: deque[tuple[int, int, int, int]] = deque()
        seen: dict[int, set[tuple[int, int]]] = defaultdict(set)
        key_set: set[str] = set()
        lock_set: set[str] = set()
        all_keys = 0
        WALL = "#"
        STARTING = "@"

        for r in range(row_count):
            for c in range(col_count):
                cell = grid[r][c]
                if cell in "abcdef":
                    all_keys += self.shift_for_all_keys(cell, "a")
                    key_set.add(cell)
                elif cell in "ABCDEF":
                    lock_set.add(cell)
                elif cell == STARTING:
                    queue.append((r, c, 0, 0))
                    seen[0].add((r, c))

        while queue:
            cur_r, cur_c, keys, dist = queue.popleft()
            for dr, dc in ((0, 1), (1, 0), (-1, 0), (0, -1)):
                new_r = cur_r + dr
                new_c = cur_c + dc
                if (new_r < 0) or (new_r >= row_count):
                    continue
                elif (new_c < 0) or (new_c >= col_count):
                    continue

                cell = grid[new_r][new_c]
                if cell == WALL:
                    continue

                if (cell in key_set) and not (
                    self.shift_for_all_keys(cell, "a") & keys
                ):
                    new_keys = keys | self.shift_for_all_keys(cell, "a")
                    if new_keys == all_keys:
                        return dist + 1

                    seen[new_keys].add((new_r, new_c))
                    queue.append((new_r, new_c, new_keys, dist + 1))
                    continue

                if (cell in lock_set) and not (
                    self.shift_for_all_keys(cell, "A") & keys
                ):
                    continue

                if (new_r, new_c) not in seen[keys]:
                    seen[keys].add((new_r, new_c))
                    queue.append((new_r, new_c, keys, dist + 1))
        return -1


def main():
    inputs = [["@.a..", "###.#", "b.A.B"], ["@..aA", "..B#.", "....b"], ["@Aa"]]

    for grid in inputs:
        solution = Solution()
        result = solution.shortestPathAllKeys(grid)
        print(result)


if __name__ == "__main__":
    main()
