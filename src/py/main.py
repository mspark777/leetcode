from __future__ import annotations
from typing import List
from collections import defaultdict
from queue import Queue


class Solution:
    def findCheapestPrice(
        self, n: int, flights: List[List[int]], src: int, dst: int, k: int
    ) -> int:
        adjacents = defaultdict[int, list[tuple[int, int]]](list)
        for flight in flights:
            adjacents[flight[0]].append((flight[1], flight[2]))

        dist = [-1] * n
        dist[src] = 0

        queue = Queue[tuple[int, int]]()
        queue.put((src, 0))
        stops = 0

        while not queue.empty() and stops <= k:
            nodes = queue.qsize()
            for _ in range(nodes):
                node, distance = queue.get()

                if node not in adjacents:
                    continue

                for neighbour, price in adjacents[node]:
                    cost = price + distance
                    if dist[neighbour] < 0 or cost < dist[neighbour]:
                        dist[neighbour] = cost
                        queue.put((neighbour, cost))

            stops += 1

        return dist[dst] if dist[dst] > 0 else -1


def main():
    input = (
        (4, [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], 0, 3, 1),
        (3, [[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1),
        (3, [[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0),
    )

    for n, flights, src, dst, k in input:
        result = Solution().findCheapestPrice(n, flights, src, dst, k)
        print(result)


if __name__ == "__main__":
    main()
