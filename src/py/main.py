from __future__ import annotations
from typing import List
from math import ceil


class Solution:
    def maximumGap(self, nums: List[int]) -> int:
        min_num = min(nums)
        max_num = max(nums)
        if min_num == max_num:
            return 0

        n = len(nums)
        bucket_len = ceil((max_num - min_num) / (n - 1))
        MAX = 10000000000
        min_bucket = [MAX] * n
        max_bucket = [-1] * n
        for num in nums:
            idx = (num - min_num) // bucket_len
            min_bucket[idx] = min(min_bucket[idx], num)
            max_bucket[idx] = max(max_bucket[idx], num)

        max_gap = bucket_len
        prev = max_bucket[0]
        for i in range(1, n):
            if min_bucket[i] == MAX:
                continue

            max_gap = max(max_gap, min_bucket[i] - prev)
            prev = max_bucket[i]

        return max_gap


def main():
    inputs = (
        [3, 6, 9, 1],
        [10],
        [
            494767408,
            862126209,
            213511142,
            768240025,
            631263193,
            839199222,
            990848796,
            214568815,
            540853864,
            760724418,
            980162605,
            976743981,
            168965760,
            680875496,
            256709197,
            970953816,
            948680062,
            347254784,
            732201399,
            786249847,
            782805044,
            40906641,
            674241381,
            784330934,
            175502610,
            731105392,
            424650824,
            549764101,
            986090032,
            710542549,
            249208107,
            448419816,
            527708664,
            158499246,
            223421723,
            114552457,
            466978424,
            821491411,
            19614107,
            115389497,
            902211438,
            2644108,
            744489871,
            897895073,
            372311214,
            551142753,
            933294894,
            426217662,
            217504874,
            983488406,
            516890023,
            426853110,
            971124103,
        ],
    )

    for nums in inputs:
        result = Solution().maximumGap(nums)
        print(result)


if __name__ == "__main__":
    main()
