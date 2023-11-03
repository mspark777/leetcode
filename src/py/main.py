from __future__ import annotations
from typing import List


class Solution:
    def buildArray(self, target: List[int], n: int) -> List[str]:
        result: list[str] = []

        cur = 1

        for num in target:
            while cur < num:
                result.append("Push")
                result.append("Pop")
                cur += 1

            result.append("Push")
            cur += 1

        return result


def main():
    inputs = (([1, 3], 3), ([1, 2, 3], 3), ([1, 2], 4))

    for target, n in inputs:
        result = Solution().buildArray(target, n)
        print(result)


if __name__ == "__main__":
    main()
