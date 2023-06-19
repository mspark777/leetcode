from typing import List


class Solution:
    def largestAltitude(self, gain: List[int]) -> int:
        result = 0
        current = result

        for altitude in gain:
            current += altitude
            result = max(result, current)

        return result


def main():
    inputs = [[-5, 1, 5, 0, -7], [-4, -3, -2, -1, 4, 3, 2]]

    for gain in inputs:
        solution = Solution()
        result = solution.largestAltitude(gain)
        print(result)


if __name__ == "__main__":
    main()
