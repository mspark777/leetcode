from typing import List


class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        result = [[0 for c in range(n)] for r in range(n)]
        cnt = 1
        dir = ((0, 1), (1, 0), (0, -1), (-1, 0))
        d = 0
        row = 0
        col = 0

        while cnt <= (n * n):
            result[row][col] = cnt
            cnt += 1

            r = self.floorMod(row + dir[d][0], n)
            c = self.floorMod(col + dir[d][1], n)

            if result[r][c] != 0:
                d = (d + 1) % 4

            row += dir[d][0]
            col += dir[d][1]

        return result

    def floorMod(self, x: int, y: int) -> int:
        return ((x % y) + y) % y


def main():
    inputs = [3, 1]

    for n in inputs:
        solution = Solution()
        result = solution.generateMatrix(n)
        print(result)


if __name__ == "__main__":
    main()
