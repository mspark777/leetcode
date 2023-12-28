from __future__ import annotations


class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        xor = x ^ y
        result = 0

        while xor != 0:
            if xor & 1 == 1:
                result += 1
            xor >>= 1

        return result


def main():
    input = ((1, 4), (3, 1))

    for x, y in input:
        result = Solution().hammingDistance(x, y)
        print(result)


if __name__ == "__main__":
    main()
