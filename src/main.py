from __future__ import annotations
from typing import List


class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        seen = [False for _ in range(len(rooms))]
        seen[0] = True

        stack: list[int] = [0]

        while stack:
            top = stack.pop()
            for key in rooms[top]:
                if not seen[key]:
                    seen[key] = True
                    stack.append(key)

        return not (False in seen)


def main():
    inputs: list[list[list[int]]] = [[[1], [2], [3], []], [[1, 3], [3, 0, 1], [2], [0]]]

    solution = Solution()
    for rooms in inputs:
        result = solution.canVisitAllRooms(rooms)
        print(result)


if __name__ == "__main__":
    main()
