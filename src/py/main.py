from __future__ import annotations
from typing import List, Optional


def offset(c: str) -> int:
    return ord(c) - ord("a")


class Node:
    next: list[Optional[Node]]
    cnt: int

    def __init__(self):
        self.next = [None] * 26
        self.cnt = 0


class Solution:
    def sumPrefixScores(self, words: List[str]) -> List[int]:
        N = len(words)
        root = Node()
        for word in words:
            self.insert(root, word)

        result = [0] * N
        for i, word in enumerate(words):
            result[i] = self.count(root, word)

        return result

    def count(self, root: Node, s: str) -> int:
        node = root
        count = 0
        for c in s:
            i = offset(c)
            child = node.next[i]
            if child is None:
                break
            count += child.cnt
            node = child

        return count

    def insert(self, root: Node, word: str):
        node = root
        for c in word:
            i = offset(c)
            child = node.next[i]
            if child is None:
                child = Node()
                node.next[i] = child

            child.cnt += 1
            node = child


def main():
    inputs = [["abc", "ab", "bc", "b"], ["abcd"]]

    for input in inputs:
        result = Solution().sumPrefixScores(input)
        print(result)


if __name__ == "__main__":
    main()
