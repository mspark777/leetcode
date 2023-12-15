from __future__ import annotations
from typing import List


class Solution:
    def destCity(self, paths: List[List[str]]) -> str:
        city_set = set[str]()
        for path in paths:
            src = path[0]
            city_set.add(src)

        for path in paths:
            dest = path[1]
            if dest not in city_set:
                return dest

        return ""


def main():
    inputs = (
        [["London", "New York"], ["New York", "Lima"], ["Lima", "Sao Paulo"]],
        [["B", "C"], ["D", "B"], ["C", "A"]],
        [["A", "Z"]],
    )

    for paths in inputs:
        result = Solution().destCity(paths)
        print(result)


if __name__ == "__main__":
    main()
