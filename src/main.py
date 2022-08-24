"""
main
"""

from __future__ import annotations
from typing import Optional


class MyQueue:
    current: list[int]
    buffer: list[int]

    def __init__(self):
        self.current = []
        self.buffer = []

    def push(self, x: int) -> None:
        self.buffer.append(x)

    def pop(self) -> int:
        self.fill_from_buffer()
        return self.current.pop()

    def peek(self) -> int:
        self.fill_from_buffer()
        return self.current[-1]

    def empty(self) -> bool:
        size = len(self.current) + len(self.buffer)
        return size < 1

    def fill_from_buffer(self):
        current = self.current
        buffer = self.buffer
        if len(current) < 1:
            while len(buffer) > 0:
                current.append(buffer.pop())


def main():
    queue = MyQueue()
    queue.push(1)
    queue.push(2)
    print(queue.peek())
    print(queue.pop())
    print(queue.empty())


if __name__ == "__main__":
    main()
