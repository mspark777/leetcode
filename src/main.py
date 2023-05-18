from typing import List


class Solution:
    def findSmallestSetOfVertices(self, n: int, edges: List[List[int]]) -> List[int]:
        to_edges = [False for _ in range(n)]
        for edge in edges:
            to = edge[1]
            to_edges[to] = True

        result: list[int] = []
        for i, isto in enumerate(to_edges):
            if not isto:
                result.append(i)

        return result


def main():
    inputs = [
        (6, [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]),
        (5, [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]),
    ]

    for n, edges in inputs:
        solution = Solution()
        result = solution.findSmallestSetOfVertices(n, edges)
        print(result)


if __name__ == "__main__":
    main()
