"""
main
"""

from typing import Optional
from solution import Solution


class Input:
    wrods1: list[str]
    wrods2: list[str]
    def __init__(self, words1: list[str], words2: list[str]):
        self.words1 = words1
        self.words2 = words2

def main():
    inputs: list[Input] = [
            Input(
              words1 = ["amazon", "apple", "facebook", "google", "leetcode"],
              words2= ["e", "o"]
            ),
            Input(
              words1 =  ["amazon", "apple", "facebook", "google", "leetcode"],
              words2 = ["l", "e"]
            )
    ]

    sol = Solution()
    for i in inputs:
        result = sol.wordSubsets(i.words1, i.words2)
        print(result)



if __name__ == "__main__":
    main()
