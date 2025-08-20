from typing import List


class Solution:
    def containsPattern(self, arr: List[int], m: int, k: int) -> bool:
        count = 0
        for i in range(0, len(arr) - m):
            if arr[i] == arr[i + m]:
                count += 1
            else:
                count = 0

            if count == (m * (k - 1)):
                return True

        return False


class Input:
    arr: list[int]
    m: int
    k: int

    def __init__(self, arr: list[int], m: int, k: int):
        self.arr = arr
        self.m = m
        self.k = k


def main():
    inputs = [
        Input([1, 2, 4, 4, 4, 4], 1, 3),
        Input([1, 2, 1, 2, 1, 1, 1, 3], 2, 2),
        Input([1, 2, 1, 2, 1, 3], 2, 3),
    ]

    for input in inputs:
        result = Solution().containsPattern(input.arr, input.m, input.k)
        print(result)


if __name__ == "__main__":
    main()
