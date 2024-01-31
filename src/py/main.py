from __future__ import annotations
from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        result: list[int] = [0] * len(temperatures)
        stack: list[int] = []

        for i, temp in enumerate(temperatures):
            while stack and temperatures[stack[-1]] < temp:
                top = stack.pop()
                result[top] = i - top

            stack.append(i)

        return result


def main():
    input = ([73, 74, 75, 71, 69, 72, 76, 73], [30, 40, 50, 60], [30, 60, 90])

    for temperatures in input:
        result = Solution().dailyTemperatures(temperatures)
        print(result)


if __name__ == "__main__":
    main()
