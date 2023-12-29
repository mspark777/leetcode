from __future__ import annotations


class Solution:
    def findComplement(self, num: int) -> int:
        mask = ~0

        while (num & mask) != 0:
            mask <<= 1

        return ~mask ^ num


def main():
    input = (5, 1)

    for num in input:
        result = Solution().findComplement(num)
        print(result)


if __name__ == "__main__":
    main()
