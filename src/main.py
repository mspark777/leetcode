from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class StockSpanner:
    stack: list[tuple[int, int]]

    def __init__(self):
        self.stack = []

    def next(self, price: int) -> int:
        stack = self.stack
        span = 1

        while stack:
            top = stack.pop()
            p, s = top
            if p <= price:
                span += s
            else:
                stack.append(top)
                break

        stack.append((price, span))
        return span


def main():
    stockSpanner = StockSpanner()
    print(
        stockSpanner.next(100),
        stockSpanner.next(80),
        stockSpanner.next(60),
        stockSpanner.next(70),
        stockSpanner.next(60),
        stockSpanner.next(75),
        stockSpanner.next(85),
    )


if __name__ == "__main__":
    main()
