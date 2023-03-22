from __future__ import annotations
from typing import Optional


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(
        self,
        val: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
        self.val = val
        self.left = left
        self.right = right


def new_tree_node(val: int, left: TreeNode, right: TreeNode) -> TreeNode:
    return TreeNode(val, left, right)


def new_tree_left(val: int, left: TreeNode) -> TreeNode:
    return TreeNode(val, left)


def new_tree_right(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, right=right)


def new_tree_val(val: int) -> TreeNode:
    return TreeNode(val)


class ListNode:
    x: int
    next: Optional[ListNode]

    def __init__(self, x: int, next: Optional[ListNode]):
        self.val = x
        self.next = next


def new_list_node(val: int, next: Optional[ListNode]) -> ListNode:
    return ListNode(val, next)


def new_list(vals: list[int]) -> Optional[ListNode]:
    head = new_list_node(0, None)
    tail = head

    for val in vals:
        node = new_list_node(val, None)
        tail.next = node
        tail = node

    return head.next


def new_cycle_list(vals: list[int], pos: int) -> Optional[ListNode]:
    head = new_list_node(0, None)
    tail = head
    target: Optional[ListNode] = None

    for i in range(len(vals)):
        node = new_list_node(vals[i], None)
        if i == pos:
            target = node

        tail.next = node
        tail = tail.next

    tail.next = target
    return head.next


class TrieNode:
    links: list[Optional[TrieNode]]
    ended: bool

    def __init__(self):
        self.links = [None] * 26
        self.ended = False

    def get(self, ch: str) -> Optional[TrieNode]:
        i = self.get_index(ch)
        return self.links[i] if self.in_range(i) else None

    def contains_key(self, ch: str) -> bool:
        return self.get(ch) is not None

    def put(self, ch: str, node: TrieNode):
        i = self.get_index(ch)
        if self.in_range(i):
            self.links[i] = node

    def set_end(self):
        self.ended = True

    def is_end(self) -> bool:
        return self.ended

    def get_index(self, ch: str) -> int:
        return ord(ch) - ord("a")

    def in_range(self, i: int) -> bool:
        return (i >= 0) and (i < len(self.links))


class Trie:
    root: TrieNode

    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        node = self.root
        for ch in word:
            if node is not None:
                if not node.contains_key(ch):
                    node.put(ch, TrieNode())

                node = node.get(ch)

        if node is not None:
            node.set_end()

    def search(self, word: str) -> bool:
        node = self.search_prefix(word)
        return (node is not None) and node.is_end()

    def startsWith(self, prefix: str) -> bool:
        return self.search_prefix(prefix) is not None

    def search_prefix(self, prefix: str) -> Optional[TrieNode]:
        node = self.root
        for ch in prefix:
            node = node.get(ch)
            if node is None:
                return None

        return node


class UnionFind:
    parents: list[int]
    ranks: list[int]

    def __init__(self, size: int):
        self.parents = [i for i in range(size)]
        self.ranks = [0 for _ in range(size)]

    def find(self, x: int) -> int:
        if self.parents[x] != x:
            self.parents[x] = self.find(self.parents[x])

        return self.parents[x]

    def union(self, x: int, y: int):
        xset = self.find(x)
        yset = self.find(y)
        if xset == yset:
            return

        if self.ranks[xset] < self.ranks[yset]:
            self.parents[xset] = yset
        elif self.ranks[xset] > self.ranks[yset]:
            self.parents[yset] = xset
        else:
            self.parents[yset] = xset
            self.ranks[xset] += 1
