"""
main
"""

from typing import Optional

class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        total = m + n - 2
        r = min(m, n) - 1

        steps = 1

        for i in range(1, r + 1, 1):
            steps = (steps * total) // i
            total -= 1

        return steps


class Input:
    m: int
    n: int
    def __init__(self, m: int, n: int):
        self.m = m
        self.n = n

def main():
    inputs: list[Input] = [
            Input(3, 7),
            Input(3, 2)
    ]

    sol = Solution()
    for i in inputs:
        m = i.m
        n = i.n
        result = sol.uniquePaths(m, n)
        print(result)




if __name__ == "__main__":
    main()
