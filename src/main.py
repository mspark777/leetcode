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
            Input([1, 2, 3, 1]),
            Input([1, 2, 3, 4]),
            Input([1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.containsDuplicate(i.nums)
        print(result)



if __name__ == "__main__":
    main()
