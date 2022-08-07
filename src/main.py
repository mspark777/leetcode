"""
main
"""

from typing import Optional

class Solution:
    def countVowelPermutation(self, n: int) -> int:
        MOD = 1000000007
        a, e, i, o, u = 1, 1, 1, 1, 1

        for j in range(n - 1):
            nexta = e + i + u
            nexte = a + i
            nexti = e + o
            nexto = i
            nextu = i + o

            a = nexta % MOD
            e = nexte % MOD
            i = nexti % MOD
            o = nexto % MOD
            u = nextu % MOD

        return (a + e + i + o + u) % MOD



class Input:
    n: int
    def __init__(self, n: int):
        self.n = n

def main():
    inputs: list[Input] = [
            Input(1),
            Input(2),
            Input(5),
            Input(144),
    ]

    s = Solution()
    for i in inputs:
        result = s.countVowelPermutation(i.n)
        print(result)




if __name__ == "__main__":
    main()
