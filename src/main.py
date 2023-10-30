from __future__ import annotations
from typing import Optional, List


class Node:
    val: int
    minval: int
    next: Optional[Node]

    def __init__(self, val: int, minval: int, next: Optional[Node]):
        self.val = val
        self.minval = minval
        self.next = next


class MinStack:
    head: Optional[Node]

    def __init__(self):
        self.head = None

    def push(self, val: int) -> None:
        if self.head is None:
            self.head = Node(val, val, None)
        else:
            self.head = Node(val, min(val, self.head.minval), self.head)

    def pop(self) -> None:
        if self.head is not None:
            self.head = self.head.next

    def top(self) -> int:
        return self.head.val if self.head is not None else 0

    def getMin(self) -> int:
        return self.head.minval if self.head is not None else 0


def main():
    stack = MinStack()
    stack.push(-2)
    stack.push(0)
    stack.push(-3)
    print(stack.getMin())
    stack.pop()
    print(stack.top())
    print(stack.getMin())


if __name__ == "__main__":
    main()
