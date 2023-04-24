from typing import List
from queue import PriorityQueue


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        queue = PriorityQueue[int]()

        for s in stones:
            queue.put(-s)

        while not queue.empty():
            y = queue.get()
            if queue.empty():
                return -y

            x = queue.get()
            if y != x:
                queue.put(y - x)

        return 0


def main():
    inputs: list[list[int]] = [[2, 7, 4, 1, 8, 1], [1]]

    for stones in inputs:
        solution = Solution()
        result = solution.lastStoneWeight(stones)
        print(result)


if __name__ == "__main__":
    main()
