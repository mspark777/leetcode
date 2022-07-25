"""
main
"""

from solution import Solution

class Input:
    nums: list[int]
    target: int
    def __init__(self, nums: list[int], target: int):
        self.nums = nums
        self.target = target

def main():
    inputs = [
            Input([5, 7, 7, 8, 8, 10], 8),
            Input([5, 7, 7, 8, 8, 10], 6),
            Input([],  0),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.searchRange(i.nums, i.target)
        print(result)



if __name__ == "__main__":
    main()
