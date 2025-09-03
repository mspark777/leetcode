from typing import List


class Solution:
    def maximumPopulation(self, logs: List[List[int]]) -> int:
        years = [0] * 101
        for birth, death in logs:
            for year in range(birth, death):
                years[year - 1950] += 1

        return years.index(max(years)) + 1950


class Input:
    logs: list[list[int]]

    def __init__(self, logs: list[list[int]]):
        self.logs = logs


def main():
    inputs = [
        Input([[1993, 1999], [2000, 2010]]),
        Input([[1950, 1961], [1960, 1971], [1970, 1981]]),
    ]

    for input in inputs:
        result = Solution().maximumPopulation(input.logs)
        print(result)


if __name__ == "__main__":
    main()
