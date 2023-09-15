from __future__ import annotations
from queue import PriorityQueue


class Solution:
    def minCostConnectPoints(self, points: list[list[int]]) -> int:
        num_points = len(points)
        visited = [False for _ in range(num_points)]
        queue: PriorityQueue[tuple[int, int]] = PriorityQueue()
        result = 0
        heap_dict: dict[int, int] = {0: 0}

        queue.put((0, 0))

        while not queue.empty():
            w, u = queue.get()

            if visited[u] or (heap_dict.get(u, float("inf")) < w):
                continue

            visited[u] = True
            result += w

            for v in range(num_points):
                if not visited[v]:
                    new_distance = self.distance(points[u], points[v])

                    if new_distance < heap_dict.get(v, float("inf")):
                        heap_dict[v] = new_distance
                        queue.put((new_distance, v))

        return result

    def distance(self, p1: list[int], p2: list[int]) -> int:
        return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])


def main():
    inputs = [[[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]], [[3, 12], [-2, 5], [-4, 1]]]

    for points in inputs:
        solution = Solution()
        result = solution.minCostConnectPoints(points)
        print(result)


if __name__ == "__main__":
    main()
