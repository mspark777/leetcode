from typing import List


class Solution:
    def compress(self, chars: List[str]) -> int:
        newlen = 0
        charslen = len(chars)
        i = 0
        while i < charslen:
            group_len = 1

            j = i + group_len
            while j < charslen:
                if chars[i] == chars[j]:
                    group_len += 1
                    j += 1
                else:
                    break

            chars[newlen] = chars[i]
            newlen += 1

            if group_len > 1:
                for ch in str(group_len):
                    chars[newlen] = ch
                    newlen += 1

            i += group_len

        return newlen


def main():
    inputs: list[list[str]] = [
        ["a", "a", "b", "b", "c", "c", "c"],
        ["a"],
        ["a", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b"],
    ]

    for chars in inputs:
        solution = Solution()
        result = solution.compress(chars)
        print(result, chars)


if __name__ == "__main__":
    main()
