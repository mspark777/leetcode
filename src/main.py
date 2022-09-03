"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible


class StackNode:
    len: int
    num: int
    digit: int

    def __init__(self, len: int, num: int, digit: int):
        self.len = len
        self.num = num
        self.digit = digit


class Solution:
    def numsSameConsecDiff(self, n: int, k: int) -> list[int]:
        stack: list[StackNode] = []
        result: list[int] = []

        for i in range(1, 10):
            node = StackNode(n - 1, i, i)
            stack.append(node)

        while len(stack) > 0:
            top = stack.pop()
            if top.len == 0:
                result.append(top.num)
                continue

            for i in range(10):
                if abs(i - top.digit) == k:
                    node = StackNode(top.len - 1, top.num * 10 + i, i)
                    stack.append(node)

        return result


class Input:
    n: int
    k: int

    def __init__(self, n: int, k: int):
        self.n = n
        self.k = k


def main():
    inputs: list[Input] = [Input(3, 7), Input(2, 1)]

    solution = Solution()
    for input in inputs:
        n = input.n
        k = input.k
        result = solution.numsSameConsecDiff(n, k)
        print(result)


if __name__ == "__main__":
    main()
