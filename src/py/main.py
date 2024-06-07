from __future__ import annotations
from typing import List, Optional


class TrieNode:
    is_end: bool
    children: list[Optional[TrieNode]]

    def __init__(self):
        self.is_end = False
        self.children = [None] * 26


class Trie:
    root: TrieNode

    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str):
        node = self.root
        for c in word:
            code = ord(c) - ord("a")
            child = node.children[code]
            if child is None:
                child = TrieNode()
                node.children[code] = child
                node = child
            else:
                node = child
        node.is_end = True

    def shortest_root(self, word: str) -> str:
        node = self.root
        for i, ch in enumerate(word):
            code = ord(ch) - ord("a")
            child = node.children[code]
            if child is None:
                return word

            node = child
            if node.is_end:
                return word[: i + 1]

        return word


class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        trie = Trie()
        for word in dictionary:
            trie.insert(word)

        return " ".join([trie.shortest_root(word) for word in sentence.split()])


def main():
    input: list[tuple[list[str], str]] = [
        (["cat", "bat", "rat"], "the cattle was rattled by the battery"),
        (["a", "b", "c"], "aadsfasf absbs bbab cadsfafs"),
    ]

    for dictionary, sentence in input:
        result = Solution().replaceWords(dictionary, sentence)
        print(result)


if __name__ == "__main__":
    main()
