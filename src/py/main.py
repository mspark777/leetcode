from __future__ import annotations


class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        n ^= n >> 16
        n ^= n >> 8
        n ^= n >> 4
        n ^= n >> 2
        n ^= n >> 1
        return n


def main():
    inputs = (3, 6)

    for n in inputs:
        result = Solution().minimumOneBitOperations(n)
        print(result)


if __name__ == "__main__":
    main()
