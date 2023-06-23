from typing import List


class Solution:
    def longestArithSeqLength(self, nums: List[int]) -> int:
        result = 0
        dp: list[dict[int, int]] = []
        for right in range(len(nums)):
            dp.append(dict())
            for left in range(right):
                diff = nums[left] - nums[right]
                lmap = dp[left]
                rmap = dp[right]

                cur_len = lmap.get(diff)
                if cur_len is None:
                    cur_len = 1
                cur_len += 1
                rmap[diff] = cur_len
                result = max(result, cur_len)

        return result


def main():
    inputs = [[3, 6, 9, 12], [9, 4, 7, 2, 10], [20, 1, 15, 3, 10, 5, 8]]

    for nums in inputs:
        solution = Solution()
        result = solution.longestArithSeqLength(nums)
        print(result)


if __name__ == "__main__":
    main()
