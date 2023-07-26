from __future__ import annotations


class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        return str(int(num1) * int(num2))


def main():
    inputs = [("2", "3"), ("123", "456")]

    for num1, num2 in inputs:
        solution = Solution()
        result = solution.multiply(num1, num2)
        print(result)


if __name__ == "__main__":
    main()
