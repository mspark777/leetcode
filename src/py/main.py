from __future__ import annotations
from itertools import zip_longest


class Solution:
    def compareVersion(self, version1: str, version2: str) -> int:
        ver1 = list(map(int, version1.split(".")))
        ver2 = list(map(int, version2.split(".")))

        for v1, v2 in zip_longest(ver1, ver2, fillvalue=0):
            if v1 < v2:
                return -1
            elif v1 > v2:
                return 1

        return 0


def main():
    inputs = (("1.01", "1.001"), ("1.0", "1.0.0"), ("0.1", "1.1"))

    for version1, version2 in inputs:
        result = Solution().compareVersion(version1, version2)
        print(result)


if __name__ == "__main__":
    main()
