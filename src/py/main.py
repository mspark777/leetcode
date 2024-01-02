from __future__ import annotations
from typing import List
from math import sqrt


class Solution:
    def constructRectangle(self, area: int) -> List[int]:
        w = int(sqrt(area))
        while area % w != 0:
            w -= 1

        return [area // w, w]


def main():
    input = (4, 37, 122122)

    for area in input:
        result = Solution().constructRectangle(area)
        print(result)


if __name__ == "__main__":
    main()
