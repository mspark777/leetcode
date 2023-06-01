from typing import List


class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        VISIT = 1
        NOT_FOUND = -1

        dx = len(grid[0]) - 1
        dy = len(grid) - 1
        if grid[dy][dx] == VISIT:
            return NOT_FOUND
        nexts: list[tuple[int, int]] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]

        MAX: int = 2**31
        result = MAX
        queue: list[tuple[int, int, int]] = [(0, 0, 1)]
        while queue:
            cx, cy, clen = queue.pop(0)
            if self.reached(cx, cy, dx, dy):
                result = min(result, clen)
                continue
            elif grid[cy][cx] == VISIT:
                continue

            nlen = clen + 1
            if nlen > result:
                continue

            grid[cy][cx] = VISIT

            for nx, ny in nexts:
                x = cx + nx
                y = cy + ny
                if not self.out_range(x, y, dx, dy):
                    queue.append((x, y, nlen))

        return result if result < MAX else NOT_FOUND

    def reached(self, x: int, y: int, dx: int, dy: int) -> bool:
        return (x == dx) and (y == dy)

    def out_range(self, x: int, y: int, dx: int, dy: int) -> bool:
        return (x < 0) or (y < 0) or (x > dx) or (y > dy)


def main():
    inputs = [
        [[0, 1], [1, 0]],
        [[0, 0, 0], [1, 1, 0], [1, 1, 0]],
        [[1, 0, 0], [1, 1, 0], [1, 1, 0]],
        [[0, 0, 0], [1, 1, 0], [1, 1, 1]],
    ]

    for grid in inputs:
        solution = Solution()
        result = solution.shortestPathBinaryMatrix(grid)
        print(result)


if __name__ == "__main__":
    main()
