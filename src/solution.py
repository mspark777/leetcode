"""
solution
"""
from __future__ import annotations

class Pair(object):
    n: int
    i: int
    def __init__(self, n: int, i: int):
        self.i = i
        self.n = n


def merge(pairs: list[Pair], l: int, mid: int, r: int, result: list[int]):
        i = l
        j = mid + 1
        temp: list[Pair] = []
        count = 0

        while (i <= mid) and (j <= r):
            ip = pairs[i]
            jp = pairs[j]
            if ip.n <= jp.n:
                result[ip.i] += count
                temp.append(ip)
                i += 1
            else:
                count += 1
                temp.append(jp)
                j += 1

        while i <= mid:
            p = pairs[i]
            result[p.i] += count
            temp.append(p)
            i += 1

        while j <= r:
            temp.append(pairs[j])
            j += 1

        for ii, val in enumerate(temp):
            pairs[l + ii] = val

def mergesort(pairs: list[Pair], l: int, r: int, result: list[int]):
    if l >= r:
        return

    mid = (l + r) // 2
    mergesort(pairs, l, mid, result)
    mergesort(pairs, mid + 1, r, result)
    merge(pairs, l, mid, r, result)


class Solution:
    def countSmaller(self, nums: list[int]) -> list[int]:
        n = len(nums)
        result = [0] * n
        pairs = [Pair(nums[i], i) for i in range(n)]

        mergesort(pairs, 0, n - 1, result)
        return result
