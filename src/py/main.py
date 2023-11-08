from __future__ import annotations


class Solution:
    def isReachableAtTime(self, sx: int, sy: int, fx: int, fy: int, t: int) -> bool:
        w = abs(sx - fx)
        h = abs(sy - fy)

        if w == 0 and h == 0 and t == 1:
            return False

        return t >= max(w, h)


def main():
    inputs = ((2, 4, 7, 7, 6), (3, 1, 7, 3, 3))

    for sx, sy, fx, fy, t in inputs:
        result = Solution().isReachableAtTime(sx, sy, fx, fy, t)
        print(result)


if __name__ == "__main__":
    main()
