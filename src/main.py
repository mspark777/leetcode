from __future__ import annotations
from typing import Optional, List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        map: dict[str, list[str]] = {}

        for str in strs:
            key = "".join(sorted(str))

            if key in map:
                map[key].append(str)
            else:
                map[key] = [str]

        return list(map.values())


def main():
    inputs: list[list[str]] = [["eat", "tea", "tan", "ate", "nat", "bat"], [""], ["a"]]

    solution = Solution()
    for strs in inputs:
        result = solution.groupAnagrams(strs)
        print(result)


if __name__ == "__main__":
    main()
