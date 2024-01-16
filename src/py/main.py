from __future__ import annotations


class Solution:
    def findLUSlength(self, a: str, b: str) -> int:
        alen = len(a)
        blen = len(b)
        return max(alen, blen) if a != b else -1


def main():
    input = (
        ("aba", "cdc"),
        ("aaa", "bbb"),
        ("aaa", "aaa"),
    )

    for a, b in input:
        result = Solution().findLUSlength(a, b)
        print(result)


if __name__ == "__main__":
    main()
