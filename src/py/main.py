from __future__ import annotations
from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack: list[int] = []

        for token in tokens:
            if token == "+":
                left, right = self.pop(stack)
                stack.append(left + right)
            elif token == "-":
                left, right = self.pop(stack)
                stack.append(left - right)
            elif token == "*":
                left, right = self.pop(stack)
                stack.append(left * right)
            elif token == "/":
                left, right = self.pop(stack)
                stack.append(int(left / right))
            else:
                stack.append(int(token))

        return stack[0]

    def pop(self, stack: list[int]) -> tuple[int, int]:
        right = stack.pop()
        left = stack.pop()
        return (left, right)


def main():
    inputs = (
        ["2", "1", "+", "3", "*"],
        ["4", "13", "5", "/", "+"],
        ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"],
    )

    for tokens in inputs:
        result = Solution().evalRPN(tokens)
        print(result)


if __name__ == "__main__":
    main()
