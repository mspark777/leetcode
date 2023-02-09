from __future__ import annotations
from typing import List


class Solution:
    def distinctNames(self, ideas: List[str]) -> int:
        group_map: dict[str, set[str]] = {}

        for idea in ideas:
            first = idea[0]
            remains = idea[1:]
            group = group_map.get(first, set())
            group.add(remains)
            group_map[first] = group

        result = 0
        groups = list(group_map.values())

        for i in range(len(groups) - 1):
            cur = groups[i]
            for j in range(i + 1, len(groups)):
                group = groups[j]
                num = 0

                for idea in cur:
                    if idea in group:
                        num += 1

                result += 2 * (len(cur) - num) * (len(group) - num)

        return result


def main():
    inputs: list[list[str]] = [["coffee", "donuts", "time", "toffee"], ["lack", "back"]]

    for ideas in inputs:
        solution = Solution()
        result = solution.distinctNames(ideas)
        print(result)


if __name__ == "__main__":
    main()
