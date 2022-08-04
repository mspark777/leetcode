"""
main
"""

from typing import Optional

class Solution:
    def mirrorReflection(self, p: int, q: int) -> int:
        while ((p % 2) + (q % 2)) == 0:
            p //= 2
            q //= 2

        return (q % 2) - (p % 2) + 1
        


class Input:
    q: int
    p: int
    def __init__(self, p: int, q: int):
        self.p = p
        self.q = q

def main():
    inputs: list[Input] = [
            Input(2, 1),
            Input(3, 1),
    ]

    s = Solution()
    for i in inputs:
        result = s.mirrorReflection(i.p, i.q)
        print(result)




if __name__ == "__main__":
    main()
