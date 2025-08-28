from typing import List


class Solution:
    def maximumWealth(self, accounts: List[List[int]]) -> int:
        return max(map(sum, accounts))


class Input:
    accounts: list[list[int]]

    def __init__(self, accounts: list[list[int]]):
        self.accounts = accounts


def main():
    inputs = [
        Input([[1, 2, 3], [3, 2, 1]]),
        Input([[1, 5], [7, 3], [3, 5]]),
        Input([[2, 8, 7], [7, 1, 3], [1, 9, 5]]),
    ]

    for input in inputs:
        result = Solution().maximumWealth(input.accounts)
        print(result)


if __name__ == "__main__":
    main()
