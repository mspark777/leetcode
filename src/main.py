from __future__ import annotations
from typing import List


class Solution:
    result: int

    def __init__(self):
        self.result = 1

    def longestPath(self, parent: List[int], s: str) -> int:
        children = [[] for i in range(len(parent))]
        for i in range(1, len(parent)):
            children[parent[i]].append(i)

        self.dfs(0, children, s)
        return self.result

    def dfs(self, current: int, children: list[list[int]], s: str) -> int:
        longest_chain = 0
        second_longest_chain = 0
        for child in children[current]:
            longest_chain_from_child = self.dfs(child, children, s)
            if s[current] == s[child]:
                continue

            if longest_chain_from_child > longest_chain:
                second_longest_chain = longest_chain
                longest_chain = longest_chain_from_child
            elif longest_chain_from_child > second_longest_chain:
                second_longest_chain = longest_chain_from_child

        self.result = max(self.result, longest_chain + second_longest_chain + 1)
        return longest_chain + 1


class Input:
    parent: list[int]
    s: str

    def __init__(self, parent: list[int], s: str):
        self.parent = parent
        self.s = s


def main():
    inputs: list[Input] = [
        Input([-1, 0, 0, 1, 1, 2], "abacbe"),
        Input([-1, 0, 0, 0], "aabc"),
    ]

    for input in inputs:
        solution = Solution()
        result = solution.longestPath(input.parent, input.s)
        print(result)


if __name__ == "__main__":
    main()
