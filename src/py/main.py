from __future__ import annotations


class Solution:
    def countSegments(self, s: str) -> int:
        n = len(s)
        result = 0
        for i in range(n):
            if (i == 0 or s[i - 1] == " ") and s[i] != " ":
                result += 1

        return result


def main():
    input = ("Hello, my name is John", "Hello", "", "                ")

    for s in input:
        result = Solution().countSegments(s)
        print(result)


if __name__ == "__main__":
    main()
