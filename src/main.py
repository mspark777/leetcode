"""
main
"""

from __future__ import annotations
from typing import Optional


class MyCircularQueue:
    queue: list[int]
    begin: int
    end: int
    size: int

    def __init__(self, k: int):
        self.queue = [0] * k
        self.begin = 0
        self.end = 0
        self.size = 0

    def enQueue(self, value: int) -> bool:
        if self.isFull():
            return False

        end = self.end
        self.queue[end] = value
        self.end = self.next_index(end)
        self.size += 1

        return True

    def deQueue(self) -> bool:
        if self.isEmpty():
            return False

        self.begin = self.next_index(self.begin)
        self.size -= 1

        return True

    def Front(self) -> int:
        return self.queue[self.begin] if not self.isEmpty() else -1

    def Rear(self) -> int:
        if self.isEmpty():
            return -1

        end = self.end
        queue = self.queue
        tail = end - 1 if end != 0 else len(queue) - 1
        return queue[tail]

    def isEmpty(self) -> bool:
        return self.size < 1

    def isFull(self) -> bool:
        return self.size >= len(self.queue)

    def next_index(self, cur: int) -> int:
        return (cur + 1) % len(self.queue)


def main():
    queue = MyCircularQueue(3)
    print(queue.enQueue(1))
    print(queue.enQueue(2))
    print(queue.enQueue(3))
    print(queue.enQueue(4))
    print(queue.Rear())
    print(queue.isFull())
    print(queue.deQueue())
    print(queue.enQueue(4))
    print(queue.Rear())


if __name__ == "__main__":
    main()
