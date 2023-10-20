from __future__ import annotations
from typing import Optional, List


class NestedInteger:
    def isInteger(self) -> bool:
        """
        @return True if this NestedInteger holds a single integer, rather than a nested list.
        """
        return False

    def getInteger(self) -> int:
        """
        @return the single integer that this NestedInteger holds, if it holds a single integer
        Return None if this NestedInteger holds a nested list
        """
        return 0

    def getList(self) -> list[NestedInteger]:
        """
        @return the nested list that this NestedInteger holds, if it holds a nested list
        Return None if this NestedInteger holds a single integer
        """
        return []


class NestedIterator:
    stack: list[NestedInteger]

    def __init__(self, nestedList: list[NestedInteger]):
        self.stack = nestedList[::-1]

    def next(self) -> int:
        return self.stack.pop().getInteger()

    def hasNext(self) -> bool:
        while self.stack:
            top = self.stack[-1]
            if top.isInteger():
                return True

            self.stack = self.stack[:-1] + top.getList()[::-1]

        return False


# Your NestedIterator object will be instantiated and called as such:
# i, v = NestedIterator(nestedList), []
# while i.hasNext(): v.append(i.next())


def main():
    inputs = ([[1, 1], 2, [1, 1]], [1, [4, [6]]])

    for s, words in inputs:
        result = Solution().wordBreak(s, words)
        print(result)


if __name__ == "__main__":
    main()
