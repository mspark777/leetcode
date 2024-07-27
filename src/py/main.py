from __future__ import annotations
from typing import List


class Solution:
    def minimumCost(
        self,
        source: str,
        target: str,
        original: List[str],
        changed: List[str],
        cost: List[int],
    ) -> int:
        result = 0
        min_cost = [[float("inf")] * 26 for _ in range(26)]

        for orig, chg, cst in zip(original, changed, cost):
            start_char = ord(orig) - ord("a")
            end_char = ord(chg) - ord("a")
            min_cost[start_char][end_char] = min(min_cost[start_char][end_char], cst)

        for k in range(26):
            for i in range(26):
                for j in range(26):
                    min_cost[i][j] = min(
                        min_cost[i][j], min_cost[i][k] + min_cost[k][j]
                    )

        for src, tgt in zip(source, target):
            if src == tgt:
                continue
            source_char = ord(src) - ord("a")
            target_char = ord(tgt) - ord("a")

            if min_cost[source_char][target_char] == float("inf"):
                return -1
            result += int(min_cost[source_char][target_char])

        return result


def main():
    inputs: list[tuple[list[int], list[int]]] = [
        ([8, 9, 4, 0, 2, 1, 3, 5, 7, 6], [991, 338, 38]),
        ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [789, 456, 123]),
    ]

    for mapping, nums in inputs:
        result = Solution().sortJumbled(mapping, nums)
        print(result)


if __name__ == "__main__":
    main()
