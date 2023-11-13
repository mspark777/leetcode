from __future__ import annotations


class Solution:
    def sortVowels(self, s: str) -> str:
        temp: list[str] = []

        for c in s:
            if self.is_vowel(c):
                temp.append(c)

        temp.sort()

        result: list[str] = []
        i = 0
        for c in s:
            if self.is_vowel(c):
                result.append(temp[i])
                i += 1
            else:
                result.append(c)

        return "".join(result)

    def is_vowel(self, c: str) -> bool:
        return c in "aeiouAEIOU"


def main():
    inputs = ("lEetcOde", "lYmpH")

    for s in inputs:
        result = Solution().sortVowels(s)
        print(result)


if __name__ == "__main__":
    main()
