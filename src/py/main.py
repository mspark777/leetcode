from __future__ import annotations


class Solution:
    def reverseStr(self, s: str, k: int) -> str:
        chs = list(s)
        for begin in range(0, len(chs), 2 * k):
            end = begin + k
            chs[begin:end] = reversed(chs[begin:end])

        return "".join(chs)


def main():
    input = (
        ("abcdefg", 2),
        ("abcd", 2),
        ("abcdefg", 3),
        (
            "hyzqyljrnigxvdtneasepfahmtyhlohwxmkqcdfehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqlimjkfnqcqnajmebeddqsgl",
            39,
        ),
    )

    for s, k in input:
        result = Solution().reverseStr(s, k)
        print(result)


if __name__ == "__main__":
    main()
