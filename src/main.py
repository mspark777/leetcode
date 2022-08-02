"""
main
"""

from typing import Optional

class Solution:
    def kthSmallest(self, matrix: list[list[int]], k: int) -> int:
        mlen = len(matrix)
        left = matrix[0][0]
        right = matrix[mlen - 1][mlen - 1]
        while left < right:
            mid = left + ((right - left) // 2)
            count = 0

            for i in range(mlen):
                for j in range(mlen - 1, -1, -1):
                    if matrix[i][j] <= mid:
                        count += j + 1
                        break

            if count < k:
                left = mid + 1
            else:
                right = mid
        return left


class Input:
    matrix: list[list[int]]
    k: int
    def __init__(self, matrix: list[list[int]], k: int):
        self.matrix = matrix
        self.k = k

def main():
    inputs: list[Input] = [
            Input([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8),
            Input([[-5]], 1),
            Input([[-5, -4], [-5, -4]], 2),
            Input([[1, 2], [1, 3]], 1)
    ]

    sol = Solution()
    for i in inputs:
        matrix = i.matrix
        k = i.k
        result = sol.kthSmallest(matrix, k)
        print(result)




if __name__ == "__main__":
    main()
