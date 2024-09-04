from __future__ import annotations
from typing import List


class Solution:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        obstacle_set = {self.hash_coordinates(x, y) for x, y in obstacles}
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

        x = 0
        y = 0
        result = 0
        current_direction = 0
        for command in commands:
            if command == -1:
                current_direction = (current_direction + 1) % 4
                continue

            if command == -2:
                current_direction = (current_direction + 3) % 4
                continue

            dx, dy = directions[current_direction]
            for _ in range(command):
                next_x, next_y = x + dx, y + dy
                if self.hash_coordinates(next_x, next_y) in obstacle_set:
                    break
                x, y = next_x, next_y

            result = max(result, x * x + y * y)

        return result

    def hash_coordinates(self, x: int, y: int) -> int:
        return x + 60001 * y


class Input:
    commands: list[int]
    obstacles: list[list[int]]

    def __init__(self, commands: list[int], obstacles: list[list[int]]):
        self.commands = commands
        self.obstacles = obstacles


def main():
    inputs = [
        Input([4, -1, 3], []),
        Input([4, -1, 4, -2, 4], [[2, 4]]),
        Input([6, -1, -1, 6], []),
    ]

    for input in inputs:
        result = Solution().robotSim(input.commands, input.obstacles)
        print(result)


if __name__ == "__main__":
    main()
