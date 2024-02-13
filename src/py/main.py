from __future__ import annotations
from typing import List


class Solution:
    def firstPalindrome(self, words: List[str]) -> str:
        for word in words:
            if self.palindrome(word):
                return word

        return ""

    def palindrome(self, word: str) -> bool:
        left = 0
        right = len(word) - 1
        while left < right:
            if word[left] != word[right]:
                return False
            else:
                left += 1
                right -= 1

        return True


def main():
    input = (
        ["abc", "car", "ada", "racecar", "cool"],
        ["notapalindrome", "racecar"],
        ["def", "ghi"],
    )

    for words in input:
        result = Solution().firstPalindrome(words)
        print(result)


if __name__ == "__main__":
    main()
