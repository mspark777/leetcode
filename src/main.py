from __future__ import annotations
from typing import Optional
from copy import deepcopy


class Node:
    val: int
    next: Optional[Node]
    random: Optional[Node]

    def __init__(
        self, x: int, next: Optional[Node] = None, random: Optional[Node] = None
    ):
        self.val = int(x)
        self.next = next
        self.random = random


class Solution:
    def copyRandomList(self, head: Optional[Node]) -> Optional[Node]:
        return deepcopy(head)


def arrtolist(pairs: list[tuple[int, Optional[int]]]) -> Optional[Node]:
    nodes: dict[int, Node] = {}

    dummy = Node(0)
    tail = dummy
    for i, (val, _) in enumerate(pairs):
        next = Node(val)
        tail.next = next
        tail = next
        nodes[i] = next

    for i, (_, r) in enumerate(pairs):
        if r is None:
            continue
        node = nodes[i]
        node.random = nodes[r]

    return dummy.next


def listtoarr(node: Optional[Node]) -> list[int]:
    pairs: list[int] = []
    while node is not None:
        pairs.append(node.val)
        node = node.next

    return pairs


def main():
    inputs = [[(7, None), (13, 0), (11, 4), (10, 2), (1, 0)], [(1, 1), (2, 1)]]

    for pairs in inputs:
        head = arrtolist(pairs)
        solution = Solution()
        result = solution.copyRandomList(head)
        print(listtoarr(result))


if __name__ == "__main__":
    main()
