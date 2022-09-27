"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def pushDominoes(self, dominoes: str) -> str:
        LEFT = "L"
        RIGHT = "R"
        STAND = "."
        LENGTH = len(dominoes)

        forces = [0 for _ in range(LENGTH)]
        force = 0
        for i, ch in enumerate(dominoes):
            if ch == LEFT:
                force = 0
            elif ch == RIGHT:
                force = LENGTH
            else:
                force = max(0, force - 1)

            forces[i] += force

        force = 0
        for i in range(LENGTH - 1, -1, -1):
            ch = dominoes[i]
            if ch == LEFT:
                force = LENGTH
            elif ch == RIGHT:
                force = 0
            else:
                force = max(0, force - 1)

            forces[i] -= force

        return "".join(
            [RIGHT if force > 0 else LEFT if force < 0 else STAND for force in forces]
        )


def main():
    inputs = ["RR.L", ".L.R...LR..L.."]

    solution = Solution()
    for input in inputs:
        result = solution.pushDominoes(input)
        print(result)


if __name__ == "__main__":
    main()
