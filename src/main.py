from __future__ import annotations


class Solution:
    def minimumDeleteSum(self, s1: str, s2: str) -> int:
        s1len = len(s1)
        s2len = len(s2)

        if s1len < s2len:
            return self.minimumDeleteSum(s2, s1)

        cur_row = [0 for _ in range(s2len + 1)]
        for i in range(1, s2len + 1):
            prev = cur_row[i - 1]
            cur_row[i] = prev + ord(s2[i - 1])

        for i in range(1, s1len + 1):
            diag = cur_row[0]
            cur_row[0] += ord(s1[i - 1])

            for j in range(1, s2len + 1):
                cur = 0
                if s1[i - 1] == s2[j - 1]:
                    cur = diag
                else:
                    lmin = ord(s1[i - 1]) + cur_row[j]
                    rmin = ord(s2[j - 1]) + cur_row[j - 1]
                    cur = min(lmin, rmin)

                diag = cur_row[j]
                cur_row[j] = cur

        return cur_row[-1]


def main():
    inputs = [("sea", "eat"), ("delete", "leet")]

    for s1, s2 in inputs:
        solution = Solution()
        result = solution.minimumDeleteSum(s1, s2)
        print(result)


if __name__ == "__main__":
    main()
