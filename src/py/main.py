from typing import List


class Solution:
    def canMakeArithmeticProgression(self, arr: List[int]) -> bool:
        arr.sort()

        for i in range(2, len(arr)):
            left = arr[i - 1] - arr[i - 2]
            right = arr[i] - arr[i - 1]
            if left != right:
                return False

        return True


class Input:
    arr: list[int]

    def __init__(self, arr: list[int]):
        self.arr = arr


def main():
    inputs = [Input([3, 5, 1]), Input([1, 2, 4])]

    for input in inputs:
        result = Solution().canMakeArithmeticProgression(input.arr)
        print(result)


if __name__ == "__main__":
    main()
