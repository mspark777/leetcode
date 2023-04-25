from queue import PriorityQueue


class SmallestInfiniteSet:
    current: int
    queue: PriorityQueue[int]
    memo: set[int]

    def __init__(self):
        self.current = 1
        self.queue = PriorityQueue()
        self.memo = set()

    def popSmallest(self) -> int:
        if self.queue.empty():
            result = self.current
            self.current += 1
            return result

        result = self.queue.get()
        self.memo.remove(result)
        return result

    def addBack(self, num: int) -> None:
        if self.current <= num:
            return
        elif num in self.memo:
            return

        self.memo.add(num)
        self.queue.put(num)


def main():
    smallestInfiniteSet = SmallestInfiniteSet()
    smallestInfiniteSet.addBack(2)
    print(smallestInfiniteSet.popSmallest())
    print(smallestInfiniteSet.popSmallest())
    print(smallestInfiniteSet.popSmallest())
    smallestInfiniteSet.addBack(1)
    print(smallestInfiniteSet.popSmallest())
    print(smallestInfiniteSet.popSmallest())
    print(smallestInfiniteSet.popSmallest())


if __name__ == "__main__":
    main()
