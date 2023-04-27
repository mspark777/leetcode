from math import sqrt


class Solution:
    def bulbSwitch(self, n: int) -> int:
        return int(sqrt(n))


def main():
    inputs = [3, 0, 1]

    for n in inputs:
        solution = Solution()
        result = solution.bulbSwitch(n)
        print(result)


if __name__ == "__main__":
    main()
