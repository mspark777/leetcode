"""
main
"""

from solution import Solution

class Input:
    nums: list[int]
    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs = [
            Input([5, 2, 6, 1]),
            Input([-1]),
            Input([-1, -1]),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.countSmaller(i.nums)
        print(result)



if __name__ == "__main__":
    main()
