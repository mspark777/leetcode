from __future__ import annotations
from typing import List


class Solution:
    def findRelativeRanks(self, score: List[int]) -> List[str]:
        ranks = [(i, score) for i, score in enumerate(score)]
        ranks.sort(key=lambda x: -x[1])

        result = ["" for _ in score]
        for rank, (ith, _) in enumerate(ranks):
            if rank == 0:
                result[ith] = "Gold Medal"
            elif rank == 1:
                result[ith] = "Silver Medal"
            elif rank == 2:
                result[ith] = "Bronze Medal"
            else:
                result[ith] = str(rank + 1)

        return result


def main():
    input = ([5, 4, 3, 2, 1], [10, 3, 8, 9, 4])

    for score in input:
        result = Solution().findRelativeRanks(score)
        print(result)


if __name__ == "__main__":
    main()
