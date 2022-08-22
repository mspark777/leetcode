"""
main
"""

from __future__ import annotations
from typing import Optional


class MyStack:
    queue: list[int]

    def __init__(self):
        self.queue = []

    def push(self, x: int) -> None:
        queue = self.queue
        queue.append(x)
        size = len(queue)
        for i in range(1, size):
            queue.append(queue.pop(0))

    def pop(self) -> int:
        return self.queue.pop(0)

    def top(self) -> int:
        return self.queue[0]

    def empty(self) -> bool:
        return len(self.queue) < 1


def main():
    my_stack = MyStack()
    my_stack.push(1)
    my_stack.push(2)
    print(my_stack.top())
    print(my_stack.pop())
    print(my_stack.empty())


if __name__ == "__main__":
    main()
