from __future__ import annotations


class Solution:
    def addBinary(self, a: str, b: str) -> str:
        result = int(a, 2) + int(b, 2)
        return bin(result)[2:]


def main():
    inputs: list[tuple[str, str]] = [("11", "1"), ("1010", "1011")]

    for a, b in inputs:
        solution = Solution()
        result = solution.addBinary(a, b)
        print(result)


if __name__ == "__main__":
    main()
