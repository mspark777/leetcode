"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def bagOfTokensScore(self, tokens: list[int], power: int) -> int:
        tokens.sort()

        score = 0
        result = 0
        i = 0
        j = len(tokens) - 1

        while (i <= j) and ((power >= tokens[i]) or (score > 0)):
            while (i <= j) and (power >= tokens[i]):
                power -= tokens[i]
                i += 1
                score += 1

            result = max(result, score)

            if (i <= j) and (score > 0):
                power += tokens[j]
                j -= 1
                score -= 1

        return result


class Input:
    tokens: list[int]
    power: int

    def __init__(self, tokens: list[int], power: int):
        self.tokens = tokens
        self.power = power


def main():
    inputs: list[Input] = [
        Input(tokens=[100], power=50),
        Input(tokens=[100, 200], power=150),
        Input(tokens=[100, 200, 300, 400], power=200),
    ]

    solution = Solution()
    for input in inputs:
        tokens = input.tokens
        power = input.power
        result = solution.bagOfTokensScore(tokens, power)
        print(result)


if __name__ == "__main__":
    main()
