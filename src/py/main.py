from typing import List


class Solution:
    def maxSatisfaction(self, satisfaction: List[int]) -> int:
        satisfaction.sort(reverse=True)

        max_satisfaction = 0
        suffix_sum = 0

        for s in satisfaction:
            suffix_sum += s
            if suffix_sum <= 0:
                break
            max_satisfaction += suffix_sum

        return max_satisfaction


def main():
    inputs: list[list[int]] = [[-1, -8, 0, 5, -9], [4, 3, 2], [-1, -4, -5]]

    for satisfaction in inputs:
        solution = Solution()
        result = solution.maxSatisfaction(satisfaction)
        print(result)


if __name__ == "__main__":
    main()
