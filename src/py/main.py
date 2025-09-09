from typing import List


class Solution:
    def findRotation(self, mat: List[List[int]], target: List[List[int]]) -> bool:
        for _ in range(4):
            if mat == target:
                return True
            mat = [list(row) for row in zip(*mat[::-1])]
        return False


class Input:
    mat: list[list[int]]
    target: list[list[int]]

    def __init__(self, mat: list[list[int]], target: list[list[int]]):
        self.mat = mat
        self.target = target


def main():
    inputs = [
        Input([[0, 1], [1, 0]], [[1, 0], [0, 1]]),
        Input([[0, 1], [1, 1]], [[1, 0], [0, 1]]),
        Input([[0, 0, 0], [0, 1, 0], [1, 1, 1]], [[1, 1, 1], [0, 1, 0], [0, 0, 0]]),
    ]

    for input in inputs:
        result = Solution().findRotation(input.mat, input.target)
        print(result)


if __name__ == "__main__":
    main()
