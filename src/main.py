"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def decodeString(self, s: str) -> str:
        stack: list[str] = []

        for ch in s:
            if ch != "]":
                stack.append(ch)
                continue

            chars: list[str] = []
            while stack[-1] != "[":
                chars.append(stack.pop())
            stack.pop()

            nums: list[str] = []
            while len(stack) > 0 and stack[-1].isdigit():
                nums.append(stack.pop())

            chars.reverse()
            nums.reverse()

            s = "".join(chars)
            count = int("".join(nums))
            stack.append(count * s)

        return "".join(stack)


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs: list[Input] = [
        Input("3[a]2[bc]"),
        Input("3[a2[c]]"),
        Input("2[abc]3[cd]ef"),
    ]

    solution = Solution()
    for input in inputs:
        s = input.s
        result = solution.decodeString(s)
        print(result)


if __name__ == "__main__":
    main()
