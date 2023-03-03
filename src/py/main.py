class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        m = len(needle)
        n = len(haystack)

        for start in range(n - m + 1):
            for i in range(m):
                if needle[i] != haystack[start + i]:
                    break

                if i == (m - 1):
                    return start

        return -1


def main():
    inputs: list[list[str]] = [["sadbutsad", "sad"], ["leetcode", "leeto"]]

    for [haystack, needle] in inputs:
        solution = Solution()
        result = solution.strStr(haystack, needle)
        print(result)


if __name__ == "__main__":
    main()
