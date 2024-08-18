from __future__ import annotations
from queue import PriorityQueue


class Solution:
    def nthUglyNumber(self, n: int) -> int:
        queue = PriorityQueue[int]()
        seen = set[int]()
        factors = [2, 3, 5]

        queue.put(1)
        seen.add(1)

        result = 1

        for _ in range(n):
            result = queue.get()
            for prime in factors:
                ugly = prime * result
                if ugly not in seen:
                    queue.put(ugly)
                    seen.add(ugly)

        return result


def main():
    inputs: list[int] = [10, 1]

    for input in inputs:
        result = Solution().nthUglyNumber(input)
        print(result)


if __name__ == "__main__":
    main()
