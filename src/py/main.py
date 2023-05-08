from typing import List


class Solution:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        result = 0
        for left in range(len(mat)):
            right = len(mat) - (left + 1)
            result += mat[left][left] + mat[right][left]

        if (len(mat) & 1) == 1:
            m = len(mat) // 2
            result -= mat[m][m]

        return result


def main():
    inputs = [
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]],
        [[5]],
    ]

    for mat in inputs:
        solution = Solution()
        result = solution.diagonalSum(mat)
        print(result)


if __name__ == "__main__":
    main()
