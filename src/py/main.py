from typing import List
import math


class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        x = int(math.sqrt(candies * 2 + 0.25) - 0.5)
        result = [0] * num_people
        for i in range(num_people):
            m = x // num_people
            if (x % num_people) > i:
                m += 1

            result[i] = m * (i + 1) + m * (m - 1) // 2 * num_people
        result[x % num_people] += candies - x * (x + 1) // 2
        return result


class Input:
    candies: int
    num_people: int

    def __init__(self, candies: int, num_people: int):
        self.candies = candies
        self.num_people = num_people


def main():
    inputs = [Input(7, 4), Input(10, 3)]

    for input in inputs:
        result = Solution().distributeCandies(input.candies, input.num_people)
        print(result)


if __name__ == "__main__":
    main()
