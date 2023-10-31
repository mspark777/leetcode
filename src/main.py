from __future__ import annotations
from typing import Optional, List


class Node:
    key: int
    val: int
    prev: Optional[Node]
    next: Optional[Node]

    def __init__(self, key: int, val: int):
        self.key = key
        self.val = val
        self.prev = None
        self.next = None


class LRUCache:
    capacity: int
    head: Node
    tail: Node
    m: dict[int, Node]

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.head = Node(-1, -1)
        self.tail = self.head
        self.head.next = self.tail
        self.tail.prev = self.head
        self.m = {}

    def add_node(self, node: Node):
        temp = self.head.next
        node.next = temp
        node.prev = self.head
        self.head.next = node
        if temp is not None:
            temp.prev = node

    def delete_node(self, node: Node):
        prevv = node.prev
        nextt = node.next
        if prevv is not None:
            prevv.next = nextt

        if nextt is not None:
            nextt.prev = prevv

    def get(self, key: int) -> int:
        if key in self.m:
            node = self.m[key]
            result = node.val
            del self.m[key]
            self.delete_node(node)
            self.add_node(node)
            if self.head.next is not None:
                self.m[key] = self.head.next
            return result

        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.m:
            curr = self.m[key]
            del self.m[key]
            self.delete_node(curr)

        if len(self.m) == self.capacity:
            if self.tail.prev is not None:
                del self.m[self.tail.prev.key]
                self.delete_node(self.tail.prev)

        self.add_node(Node(key, value))
        if self.head.next is not None:
            self.m[key] = self.head.next


def main():
    cache = LRUCache(2)
    cache.put(1, 1)
    cache.put(2, 2)
    print(cache.get(1))
    cache.put(3, 3)
    print(cache.get(2))
    cache.put(4, 4)
    print(cache.get(1))
    print(cache.get(3))
    print(cache.get(4))


if __name__ == "__main__":
    main()
