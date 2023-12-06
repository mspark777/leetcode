from __future__ import annotations


class Solution:
    def totalMoney(self, n: int) -> int:
        weeks = n // 7
        first = 28
        last = first + (weeks - 1) * 7
        arithmetic_sum = weeks * (first + last) // 2
        monday = 1 + weeks
        final_week = 0
        for day in range(n % 7):
            final_week += monday + day

        return arithmetic_sum + final_week


def main():
    inputs = (4, 10, 20)

    for n in inputs:
        result = Solution().totalMoney(n)
        print(result)


if __name__ == "__main__":
    main()
