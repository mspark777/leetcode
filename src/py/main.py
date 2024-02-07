from __future__ import annotations
from collections import Counter


class Solution:
    def frequencySort(self, s: str) -> str:
        counts = Counter(s)
        keys = list(counts.keys())
        keys.sort(key=lambda k: -counts[k])
        result: list[str] = []
        for key in keys:
            for _ in range(counts[key]):
                result.append(key)

        return "".join(result)


def main():
    input = ("tree", "cccaaa", "Aabb", "loveleetcode")

    for s in input:
        result = Solution().frequencySort(s)
        print(result)


if __name__ == "__main__":
    main()
