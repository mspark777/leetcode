from __future__ import annotations


class Solution:
    def countOdds(self, low: int, high: int) -> int:
        if (low & 1) == 0:
            low += 1

        return ((high - low) // 2) + 1 if low <= high else 0


def main():
    inputs: list[tuple[int, int]] = [(3, 7), (8, 10)]

    for low, high in inputs:
        solution = Solution()
        result = solution.countOdds(low, high)
        print(result)


if __name__ == "__main__":
    main()
