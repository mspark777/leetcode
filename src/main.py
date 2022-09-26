"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def equationsPossible(self, equations: list[str]) -> bool:
        parents = [i for i in range(27)]

        for equation in equations:
            if equation[1] == "=":
                a = parents[ord(equation[0]) - ord("a")]
                b = parents[ord(equation[3]) - ord("a")]
                self.union(parents, a, b)

        for equation in equations:
            if equation[1] == "!":
                a = parents[ord(equation[0]) - ord("a")]
                b = parents[ord(equation[3]) - ord("a")]
                if self.find(parents, a) == self.find(parents, b):
                    return False

        return True

    def find(self, parents: list[int], code: int) -> int:
        parent = parents[code]
        if parent == code:
            return code

        parent = self.find(parents, parent)
        parents[code] = parent
        return parent

    def union(self, parents: list[int], a: int, b: int):
        parenta = self.find(parents, a)
        parentb = self.find(parents, b)
        parents[parentb] = parenta


def main():
    inputs = [["a==b", "b!=a"], ["b==a", "a==b"], ["c==c", "b==d", "x!=z"]]

    solution = Solution()
    for input in inputs:
        result = solution.equationsPossible(input)
        print(result)


if __name__ == "__main__":
    main()
