from __future__ import annotations
from typing import List


class Solution:
    def readBinaryWatch(self, turned_on: int) -> List[str]:
        result: list[str] = []
        for h in range(12):
            for m in range(60):
                num = (h << 6) | m
                ones = num.bit_count()
                if ones == turned_on:
                    time = "{}:{}{}".format(h, "0" if m < 10 else "", m)
                    result.append(time)

        return result


def main():
    inputs: list[int] = [1, 9]
    for turned_on in inputs:
        solution = Solution()
        result = solution.readBinaryWatch(turned_on)
        print(result)


if __name__ == "__main__":
    main()
