"""
main
"""

from typing import Optional, SupportsComplex
from math import ceil, log

class Solution:
    def poorPigs(self, buckets: int, minutesToDie: int, minutesToTest: int) -> int:
        return ceil(log(buckets) / log(minutesToTest / minutesToDie + 1))



class Input:
    buckets: int
    minutesToDie: int
    minutesToTest: int
    def __init__(self, buckets: int, minutesToDie: int, minutesToTest: int):
        self.buckets = buckets
        self.minutesToDie = minutesToDie
        self.minutesToTest = minutesToTest

def main():
    inputs: list[Input] = [
            Input(1000, 15, 60),
            Input(4, 15, 15),
            Input(4, 15, 30),
    ]

    s = Solution()
    for i in inputs:
        result = s.poorPigs(i.buckets, i.minutesToDie, i.minutesToTest)
        print(result)




if __name__ == "__main__":
    main()
