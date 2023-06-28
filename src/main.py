from typing import List


class Solution:
    def maxProbability(
        self,
        n: int,
        edges: List[List[int]],
        succ_prob: List[float],
        start: int,
        end: int,
    ) -> float:
        max_props = [0.0] * n
        max_props[start] = 1.0

        for _ in range(n - 1):
            breakable = True
            for j, [u, v] in enumerate(edges):
                prob = succ_prob[j]
                umax = max_props[u] * prob
                if umax > max_props[v]:
                    max_props[v] = umax
                    breakable = False

                vmax = max_props[v] * prob
                if vmax > max_props[u]:
                    max_props[u] = vmax
                    breakable = False

            if breakable:
                break

        return max_props[end]


def main():
    inputs = [
        (3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.2], 0, 2),
        (3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.3], 0, 2),
        (3, [[0, 1]], [0.5], 0, 2),
        (
            5,
            [[2, 3], [1, 2], [3, 4], [1, 3], [1, 4], [0, 1], [2, 4], [0, 4], [0, 2]],
            [0.06, 0.26, 0.49, 0.25, 0.2, 0.64, 0.23, 0.21, 0.77],
            0,
            3,
        ),
    ]

    for n, edges, succ_prob, start, end in inputs:
        solution = Solution()
        result = solution.maxProbability(n, edges, succ_prob, start, end)
        print(result)


if __name__ == "__main__":
    main()
