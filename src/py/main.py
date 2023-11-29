from __future__ import annotations


class Solution:
    def hammingWeight(self, n: int) -> int:
        result = 0
        while n != 0:
            n &= n - 1
            result += 1

        return result


def main():
    inputs = (0b1011, 0b10000000, 0b11111111111111111111111111111101)

    for n in inputs:
        result = Solution().hammingWeight(n)
        print(result)


if __name__ == "__main__":
    main()
