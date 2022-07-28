"""
main
"""

from typing import Optional
from solution import Solution


class Input:
    s: str
    t: str
    def __init__(self, s: str, t: str):
        self.s = s
        self.t = t

def main():
    inputs = [
            Input(s = 'anagram', t = 'nagaram'),
            Input(s = 'rat', t = 'car'),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.isAnagram(i.s, i.t)
        print(result)



if __name__ == "__main__":
    main()
