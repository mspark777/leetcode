from __future__ import annotations


class Solution:
    def rangeBitwiseAnd(self, left: int, right: int) -> int:
        cnt = 0
        while left != right:
            left >>= 1
            right >>= 1
            cnt += 1

        return left << cnt


def main():
    input = ((5, 7), (0, 0), (1, 214783647))

    for left, right in input:
        result = Solution().rangeBitwiseAnd(left, right)
        print(result)


if __name__ == "__main__":
    main()
