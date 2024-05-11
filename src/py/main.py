from __future__ import annotations
from typing import List
import heapq


class Solution:
    def mincostToHireWorkers(
        self, quality: List[int], wage: List[int], k: int
    ) -> float:
        n = len(quality)
        current_total_quality = 0.0
        wage_to_quality_ratio = [(wage[i] / q, q) for i, q in enumerate(quality)]
        wage_to_quality_ratio.sort(key=lambda x: x[0])
        highest_quality_workers: list[int] = []
        result = float("inf")
        for i in range(n):
            heapq.heappush(highest_quality_workers, -wage_to_quality_ratio[i][1])
            current_total_quality += wage_to_quality_ratio[i][1]

            if len(highest_quality_workers) > k:
                current_total_quality += heapq.heappop(highest_quality_workers)

            if len(highest_quality_workers) == k:
                result = min(
                    result, current_total_quality * wage_to_quality_ratio[i][0]
                )

        return result


def main():
    input: list[tuple[list[int], list[int], int]] = [
        ([10, 20, 5], [70, 50, 30], 2),
        ([3, 1, 10, 10, 1], [4, 8, 2, 2, 7], 3),
    ]

    for quality, wage, k in input:
        result = Solution().mincostToHireWorkers(quality, wage, k)
        print(result)


if __name__ == "__main__":
    main()
