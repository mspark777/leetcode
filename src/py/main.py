from typing import Optional, List


class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:
        count = 0
        last = len(flowerbed) - 1
        for i in range(0, last + 1):
            plot = flowerbed[i]
            if plot == 1:
                continue

            empty_left = (i == 0) or (flowerbed[i - 1] == 0)
            empty_right = (i == last) or (flowerbed[i + 1] == 0)
            if empty_left and empty_right:
                flowerbed[i] = 1
                count += 1
                if count >= n:
                    return True

        return count >= n


def main():
    inputs: list[tuple[list[int], int]] = [([1, 0, 0, 0, 1], 1), ([1, 0, 0, 0, 1], 2)]

    for flowerbed, n in inputs:
        solution = Solution()
        result = solution.canPlaceFlowers(flowerbed, n)
        print(result)


if __name__ == "__main__":
    main()
