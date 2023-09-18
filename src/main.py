from __future__ import annotations


class Solution:
    def kWeakestRows(self, mat: list[list[int]], k: int) -> list[int]:
        rows = [(sum(row), i) for i, row in enumerate(mat)]
        rows.sort(key=lambda x: (x[0], x[1]))
        return [x[1] for x in rows[:k]]


def main():
    inputs = [
        (
            [
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 1],
            ],
            3,
        ),
        ([[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]], 2),
    ]

    for mat, k in inputs:
        solution = Solution()
        result = solution.kWeakestRows(mat, k)
        print(result)


if __name__ == "__main__":
    main()
