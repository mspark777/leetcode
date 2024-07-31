from __future__ import annotations
from typing import List


class Solution:
    def minHeightShelves(self, books: List[List[int]], shelf_width: int) -> int:
        n = len(books)
        dp = [0] * (n + 1)

        dp[0] = 0
        dp[1] = books[0][1]

        for i in range(2, n + 1):
            remaning_shelf_width = shelf_width - books[i - 1][0]
            max_height = books[i - 1][1]
            dp[i] = books[i - 1][1] + dp[i - 1]

            j = i - 1

            while j > 0 and remaning_shelf_width - books[j - 1][0] >= 0:
                max_height = max(max_height, books[j - 1][1])
                remaning_shelf_width -= books[j - 1][0]
                dp[i] = min(dp[i], max_height + dp[j - 1])
                j -= 1

        return dp[n]


class Input:
    books: list[list[int]]
    shelf_width: int

    def __init__(self, books: list[list[int]], shelf_width: int):
        self.books = books
        self.shelf_width = shelf_width


def main():
    inputs: list[Input] = [
        Input([[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]], 4),
        Input([[1, 3], [2, 4], [3, 2]], 6),
    ]

    for input in inputs:
        result = Solution().minHeightShelves(input.books, input.shelf_width)
        print(result)


if __name__ == "__main__":
    main()
