from __future__ import annotations


class Solution:
    def candy(self, ratings: list[int]) -> int:
        if not ratings:
            return 0

        result = 1
        up = 0
        down = 0
        peak = 0

        for i in range(len(ratings) - 1):
            prev = ratings[i]
            cur = ratings[i + 1]

            if prev < cur:
                up += 1
                down = 0
                peak = up
                result += 1 + up
            elif prev == cur:
                up = 0
                down = 0
                peak = 0
                result += 1
            else:
                up = 0
                down += 1
                result += 1 + down
                if peak >= down:
                    result -= 1

        return result


def main():
    inputs = [[1, 0, 2], [1, 2, 2]]

    for ratings in inputs:
        solution = Solution()
        result = solution.candy(ratings)
        print(result)


if __name__ == "__main__":
    main()
