from __future__ import annotations


class Solution:
    def trailingZeroes(self, n: int) -> int:
        power_of_five = 5
        result = 0

        while True:
            zeros = n // power_of_five
            if zeros == 0:
                break

            result += zeros
            power_of_five *= 5

        return result


def main():
    inputs = (3, 5, 0)

    for n in inputs:
        result = Solution().trailingZeroes(n)
        print(result)


if __name__ == "__main__":
    main()
