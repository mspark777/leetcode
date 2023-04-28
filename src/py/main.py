from typing import List

from utils import UnionFind


class Solution:
    def numSimilarGroups(self, strs: List[str]) -> int:
        strs_len = len(strs)
        result = strs_len
        uf = UnionFind(strs_len)
        for i in range(strs_len):
            a = strs[i]
            for j in range(i + 1, strs_len):
                b = strs[j]
                if not self.is_similar(a, b):
                    continue

                if uf.find(i) != uf.find(j):
                    result -= 1
                    uf.union(i, j)

        return result

    def is_similar(self, a: str, b: str) -> bool:
        diff = 0

        for ca, cb in zip(a, b):
            if ca != cb:
                diff += 1

        return diff in [0, 2]


def main():
    inputs = [["tars", "rats", "arts", "star"], ["omv", "ovm"]]

    for strs in inputs:
        solution = Solution()
        result = solution.numSimilarGroups(strs)
        print(result)


if __name__ == "__main__":
    main()
