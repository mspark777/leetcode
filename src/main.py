from typing import List


class Solution:
    def maximumDetonation(self, bombs: List[List[int]]) -> int:
        graph: dict[int, list[int]] = {}

        for i, ibomb in enumerate(bombs):
            for j, jbomb in enumerate(bombs):
                if i == j:
                    continue

                [ix, iy, ir] = ibomb
                [jx, jy, _] = jbomb
                dx = ix - jx
                dy = iy - jy
                d = (dx * dx) + (dy * dy)
                r = ir * ir
                if r < d:
                    continue

                nodes = graph.get(i)
                if nodes is not None:
                    nodes.append(j)
                else:
                    graph[i] = [j]

        result = 0
        for i in range(len(bombs)):
            result = max(result, self.dfs(i, graph))

        return result

    def dfs(self, i: int, graph: dict[int, list[int]]) -> int:
        stack: list[int] = [i]
        visited: set[int] = set(stack)

        while stack:
            top = stack.pop()
            nodes = graph.get(top)
            if nodes is None:
                continue

            for node in nodes:
                if node not in visited:
                    stack.append(node)
                    visited.add(node)

        return len(visited)


def main():
    inputs = [
        [[2, 1, 3], [6, 1, 4]],
        [[1, 1, 5], [10, 10, 5]],
        [[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]],
    ]

    for bombs in inputs:
        solution = Solution()
        result = solution.maximumDetonation(bombs)
        print(result)


if __name__ == "__main__":
    main()
