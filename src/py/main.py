from typing import List


class Solution:
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        people.sort()
        left = 0
        right = len(people) - 1
        result = 0

        while left <= right:
            light = people[left]
            heavy = people[right]
            result += 1
            total = light + heavy
            right -= 1
            if total <= limit:
                left += 1

        return result


def main():
    inputs: list[tuple[list[int], int]] = [
        ([1, 2], 3),
        ([3, 2, 2, 1], 3),
        ([3, 5, 3, 4], 5),
    ]

    for people, limit in inputs:
        solution = Solution()
        result = solution.numRescueBoats(people, limit)
        print(result)


if __name__ == "__main__":
    main()
