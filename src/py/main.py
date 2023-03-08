from typing import List


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        sum = 0
        max_pile = 0
        for pile in piles:
            sum += pile
            max_pile = max(max_pile, pile)

        left = sum // h
        right = max_pile
        while left < right:
            middle = (left + right) // 2
            required = self.hours_required(piles, middle)
            if required > h:
                left = middle + 1
            else:
                right = middle

        return left

    def hours_required(self, piles: list[int], k: int) -> int:
        if k == 0:
            return 2**31

        hours = 0
        for pile in piles:
            if (pile % k) != 0:
                hours += 1

            hours += pile // k

        return hours


def main():
    inputs: list[tuple[list[int], int]] = [
        ([3, 6, 7, 11], 8),
        ([30, 11, 23, 4, 20], 5),
        ([30, 11, 23, 4, 20], 6),
    ]

    for piles, h in inputs:
        solution = Solution()
        result = solution.minEatingSpeed(piles, h)
        print(result)


if __name__ == "__main__":
    main()
