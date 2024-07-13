from __future__ import annotations
from typing import List


class Solution:
    def survivedRobotsHealths(
        self, positions: List[int], healths: List[int], directions: str
    ) -> List[int]:
        n = len(positions)
        indices = list(range(n))
        stack: list[int] = []

        indices.sort(key=lambda x: positions[x])

        for curr in indices:
            if directions[curr] == "R":
                stack.append(curr)
            else:
                while stack and healths[curr] > 0:
                    top = stack.pop()

                    if healths[top] > healths[curr]:
                        healths[top] -= 1
                        healths[curr] = 0
                        stack.append(top)
                    elif healths[top] < healths[curr]:
                        healths[curr] -= 1
                        healths[top] = 0
                    else:
                        healths[curr] = 0
                        healths[top] = 0

        result: list[int] = []
        for health in healths:
            if health > 0:
                result.append(health)

        return result


class Input:
    positions: list[int]
    healths: list[int]
    directions: str

    def __init__(self, positions: list[int], healths: list[int], directions: str):
        self.positions = positions
        self.healths = healths
        self.directions = directions


def main():
    inputs: list[Input] = [
        Input([5, 4, 3, 2, 1], [2, 17, 9, 15, 10], "RRRRR"),
        Input([3, 5, 2, 6], [10, 10, 15, 12], "RLRL"),
        Input([1, 2, 5, 6], [10, 10, 11, 11], "RLRL"),
    ]

    for input in inputs:
        result = Solution().survivedRobotsHealths(
            input.positions, input.healths, input.directions
        )
        print(result)


if __name__ == "__main__":
    main()
