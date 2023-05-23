from typing import List
from queue import PriorityQueue


class KthLargest:
    k: int
    queue: PriorityQueue

    def __init__(self, k: int, nums: List[int]):
        self.k = k
        self.queue = PriorityQueue()

        for num in nums:
            self.add(num)

    def add(self, val: int) -> int:
        self.queue.put(val)

        while self.queue.qsize() > self.k:
            self.queue.get()

        v = self.queue.get()
        self.queue.put(v)
        return v


def main():
    kth_largest = KthLargest(3, [4, 5, 8, 2])
    print(kth_largest.add(3))
    print(kth_largest.add(5))
    print(kth_largest.add(10))
    print(kth_largest.add(9))
    print(kth_largest.add(4))


if __name__ == "__main__":
    main()
