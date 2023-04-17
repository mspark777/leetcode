from typing import List


class Solution:
    def kidsWithCandies(self, candies: List[int], extra_candies: int) -> List[bool]:
        max_candy = max(candies) - extra_candies
        return [candy >= max_candy for candy in candies]


def main():
    inputs: list[tuple[list[int], int]] = [
        ([2, 3, 5, 1, 3], 3),
        ([4, 2, 1, 1, 2], 1),
        ([12, 1, 12], 10),
    ]

    for candies, extra_candies in inputs:
        solution = Solution()
        result = solution.kidsWithCandies(candies, extra_candies)
        print(result)


if __name__ == "__main__":
    main()
