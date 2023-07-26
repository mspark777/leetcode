from __future__ import annotations
from typing import List
from math import ceil


class Solution:
    def minSpeedOnTime(self, dist: List[int], hour: float) -> int:
        left = 1
        right = 10000001
        min_speed = -1

        while left <= right:
            mid = (left + right) // 2
            if self.time_required(dist, mid) <= hour:
                min_speed = mid
                right = mid - 1
            else:
                left = mid + 1

        return min_speed

    def time_required(self, dist: list[int], speed: int) -> float:
        time = 0.0
        for i, d in enumerate(dist):
            t = d / speed
            time += t if i == (len(dist) - 1) else ceil(t)

        return time


def main():
    inputs = [
        ([1, 3, 2], 6),
        ([1, 3, 2], 2.7),
        ([1, 3, 2], 1.9),
        ([1, 1, 100000], 2.01),
        (
            [
                90,
                94,
                72,
                85,
                92,
                63,
                20,
                25,
                38,
                28,
                8,
                75,
                95,
                70,
                8,
                96,
                41,
                8,
                7,
                75,
                62,
                65,
                68,
                21,
                8,
                66,
                11,
                24,
                9,
                77,
                9,
                87,
                31,
                52,
                16,
                73,
                32,
                75,
                77,
                6,
                80,
                11,
                54,
                85,
                75,
                73,
                67,
                41,
                34,
                27,
                86,
                92,
                41,
                31,
                34,
                51,
                17,
                86,
                83,
                39,
                59,
                41,
                97,
                10,
                2,
                59,
                80,
                73,
                13,
                10,
                69,
                28,
                65,
                34,
                17,
                45,
                9,
            ],
            393.18,
        ),
    ]

    for dist, hour in inputs:
        solution = Solution()
        result = solution.minSpeedOnTime(dist, hour)
        print(result)


if __name__ == "__main__":
    main()
