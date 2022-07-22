"""
main
"""

from typing import Optional
from solution import Solution

class Input:
    s: str
    def __init__(self, s: str):
        self.s = s

def main():
    inputs = [
            Input("A man, a plan, a canal: Panama"),
            Input("race a car"),
            Input(" "),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.isPalindrome(i.s)
        print(result)



if __name__ == "__main__":
    main()
