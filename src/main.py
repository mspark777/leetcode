from __future__ import annotations
from typing import Optional, List
from collections import defaultdict


class Solution:
    def minimumTime(self, n: int, relations: List[List[int]], time: List[int]) -> int:
        graph = defaultdict[int, list[int]](list)
        indegress = [0] * n

        for x, y in relations:
            graph[x - 1].append(y - 1)
            indegress[y - 1] += 1

        queue: list[int] = []
        max_time = [0] * n

        for node in range(n):
            if indegress[node] == 0:
                queue.append(node)
                max_time[node] = time[node]

        while queue:
            node = queue.pop(0)
            for neighbor in graph[node]:
                max_time[neighbor] = max(max_time[neighbor], max_time[node] + time[neighbor])
                indegress[neighbor] -= 1
                if indegress[neighbor] == 0:
                    queue.append(neighbor)

        return max(max_time)


def main():
    inputs = (
        (3, [[1, 3], [2, 3]], [3, 2, 5]),
        (5, [[1, 5], [2, 5], [3, 5], [3, 4], [4, 5]], [1, 2, 3, 4, 5]),
    )

    for n, relations, time in inputs:
        result = Solution().minimumTime(n, relations, time)
        print(result)


if __name__ == "__main__":
    main()
