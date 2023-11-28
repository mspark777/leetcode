from __future__ import annotations
from typing import Optional


class Solution:
    def numberOfWays(self, corridor: str) -> int:
        MOD = 1_000_000_007

        count = 1
        seats = 0

        prevoius_pair_last: Optional[int] = None

        for i, thing in enumerate(corridor):
            if thing == "S":
                seats += 1

                if seats == 2:
                    prevoius_pair_last = i
                    seats = 0
                elif seats == 1 and prevoius_pair_last is not None:
                    count *= i - prevoius_pair_last
                    count %= MOD

        if seats == 1 or prevoius_pair_last is None:
            return 0

        return count


def main():
    inputs = ("SSPPSPS", "PPSPSP", "S")

    for corridor in inputs:
        result = Solution().numberOfWays(corridor)
        print(result)


if __name__ == "__main__":
    main()
