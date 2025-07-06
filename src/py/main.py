from collections import deque
from collections import Counter


class FindSumPairs:
    nums1: list[int]
    nums2: list[int]
    counts: dict[int, int]

    def __init__(self, nums1: list[int], nums2: list[int]):
        self.nums1 = nums1
        self.nums2 = nums2
        self.counts = Counter(nums2)

    def add(self, index: int, val: int) -> None:
        self.counts[self.nums2[index]] -= 1
        self.nums2[index] += val
        self.counts[self.nums2[index]] += 1

    def count(self, tot: int) -> int:
        count = 0
        for num in self.nums1:
            count += self.counts[tot - num]

        return count


class Solution:
    def largestPathValue(self, colors: str, edges: list[list[int]]) -> int:
        n = len(colors)
        graph: list[list[int]] = [[] for _ in range(n)]
        indegree = [0] * n
        for u, v in edges:
            graph[u].append(v)
            indegree[v] += 1

        dp = [[0] * 26 for _ in range(n)]
        queue = deque()
        for i in range(n):
            if indegree[i] == 0:
                queue.append(i)
                dp[i][ord(colors[i]) - ord("a")] = 1

        visited = 0
        max_color_value = 0

        while queue:
            u = queue.popleft()
            visited += 1

            for v in graph[u]:
                for c in range(26):
                    dp[v][c] = max(
                        dp[v][c],
                        dp[u][c] + (1 if c == ord(colors[v]) - ord("a") else 0),
                    )
                indegree[v] -= 1
                if indegree[v] == 0:
                    queue.append(v)

            max_color_value = max(max_color_value, max(dp[u]))

        return max_color_value if visited == n else -1


class Input:
    colors: str
    edges: list[list[int]]

    def __init__(self, colors: str, edges: list[list[int]]):
        self.colors = colors
        self.edges = edges


def main():
    inputs = [Input("abaca", [[0, 1], [0, 2], [2, 3], [3, 4]]), Input("a", [[0, 0]])]

    for input in inputs:
        result = Solution().largestPathValue(input.colors, input.edges)
        print(result)


if __name__ == "__main__":
    main()
