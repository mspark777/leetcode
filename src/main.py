from __future__ import annotations
from typing import Optional, List


class Solution:
    def isPowerOfFour(self, n: int) -> bool:
        if n == 1:
            return True
        elif n == 0:
            return False
        elif n % 4 != 0:
            return False

        return self.isPowerOfFour(n // 4)


def main():
    inputs = (16, 5, 1)

    for n in inputs:
        result = Solution().isPowerOfFour(n)
        print(result)


if __name__ == "__main__":
    main()
