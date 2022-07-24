"""
main
"""

from solution import Solution

class Input:
    matrix: list[list[int]]
    target: int
    def __init__(self, matrix: list[list[int]], target: int):
        self.matrix = matrix
        self.target = target

def main():
    inputs = [
            Input(matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5),
            Input(matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20)
    ]
    sol = Solution()
    for i in inputs:
        result = sol.searchMatrix(i.matrix, i.target)
        print(result)



if __name__ == "__main__":
    main()
