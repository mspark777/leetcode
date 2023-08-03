from __future__ import annotations
from typing import List


class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []

        letters_map: dict[str, str] = dict()
        letters_map["2"] = "abc"
        letters_map["3"] = "def"
        letters_map["4"] = "ghi"
        letters_map["5"] = "jkl"
        letters_map["6"] = "mno"
        letters_map["7"] = "pqrs"
        letters_map["8"] = "tuv"
        letters_map["9"] = "wxyz"

        stack: list[str] = [""]
        result: list[str] = []

        while stack:
            top = stack.pop()
            ch = digits[len(top)]
            letters = letters_map[ch]
            for letter in letters:
                combination = top + letter
                if len(combination) == len(digits):
                    result.append(combination)
                else:
                    stack.append(combination)

        return result


def main():
    inputs = ["23", "", "2"]

    for digits in inputs:
        solution = Solution()
        result = solution.letterCombinations(digits)
        print(result)


if __name__ == "__main__":
    main()
