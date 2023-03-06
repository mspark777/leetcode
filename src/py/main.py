from typing import List


class Solution:
    def findKthPositive(self, arr: List[int], k: int) -> int:
        left = 0
        right = len(arr)
        while left < right:
            middle = (left + right) // 2
            n = arr[middle] - (middle + 1)
            if n < k:
                left = middle + 1
            else:
                right = middle

        return left + k


def main():
    inputs: list[tuple[list[int], int]] = [([2, 3, 4, 7, 11], 5), ([1, 2, 3, 4], 2)]

    for arr, k in inputs:
        solution = Solution()
        result = solution.findKthPositive(arr, k)
        print(result)


if __name__ == "__main__":
    main()
