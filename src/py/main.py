from typing import List


class Solution:
    count: int

    def __init__(self):
        self.count = 0

    def minReorder(self, _n: int, connections: List[List[int]]) -> int:
        adjs: dict[int, list[tuple[int, int]]] = {}
        for [a, b] in connections:
            aedges = adjs[a] if a in adjs else []
            bedges = adjs[b] if b in adjs else []

            aedges.append((b, 1))
            bedges.append((a, 0))

            adjs[a] = aedges
            adjs[b] = bedges

        return self.dfs(0, -1, adjs)

    def dfs(
        self, node: int, parent: int, adjs: dict[int, list[tuple[int, int]]]
    ) -> int:
        adj = adjs.get(node)
        if adj is None:
            return self.count

        for child, sign in adj:
            if child != parent:
                self.count += sign
                self.dfs(child, node, adjs)

        return self.count


def main():
    inputs: list[tuple[int, list[list[int]]]] = [
        (6, [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]),
        (5, [[1, 0], [1, 2], [3, 2], [3, 4]]),
        (0, [[1, 0], [2, 0]]),
    ]

    for n, connections in inputs:
        solution = Solution()
        result = solution.minReorder(n, connections)
        print(result)


if __name__ == "__main__":
    main()
