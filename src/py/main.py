from __future__ import annotations
from typing import List


class Solution:
    def maximumScoreAfterOperations(
        self, edges: List[List[int]], values: List[int]
    ) -> int:
        node_count = len(values)
        adj = [[] for _ in range(node_count)]

        for u, v in edges:
            adj[u].append(v)
            adj[v].append(u)

        subtree = [n for n in values]
        self.pre(adj, 0, -1, subtree, values)

        dp = [-1 for _ in range(node_count)]
        self.dfs(adj, 0, -1, dp, subtree, values)
        return dp[0]

    def pre(
        self,
        adj: list[list[int]],
        node: int,
        parent: int,
        subtree: list[int],
        values: list[int],
    ):
        for child in adj[node]:
            if child != parent:
                self.pre(adj, child, node, subtree, values)
                subtree[node] += subtree[child]

    def dfs(
        self,
        adj: list[list[int]],
        node: int,
        parent: int,
        dp: list[int],
        subtree: list[int],
        values: list[int],
    ):
        dp[node] = subtree[node] - values[node]
        sum = 0
        cnt = 0

        for child in adj[node]:
            if child != parent:
                self.dfs(adj, child, node, dp, subtree, values)
                cnt += 1
                sum += dp[child]

        if cnt > 0:
            dp[node] = max(dp[node], values[node] + sum)


def main():
    input = (
        ([[0, 1], [0, 2], [0, 3], [2, 4], [4, 5]], [5, 2, 5, 2, 1, 1]),
        ([[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]], [20, 10, 9, 7, 4, 3, 5]),
    )

    for edges, values in input:
        result = Solution().maximumScoreAfterOperations(edges, values)
        print(result)


if __name__ == "__main__":
    main()
