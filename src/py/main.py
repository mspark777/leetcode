from __future__ import annotations


class Solution:
    def isPathCrossing(self, path: str) -> bool:
        x = 0
        y = 0
        visit = set[tuple[int, int]]([(x, y)])

        for dir in path:
            next_x = x - 1
            next_y = y
            if dir == "N":
                next_x = x
                next_y = y + 1
            elif dir == "E":
                next_x = x + 1
                next_y = y
            elif dir == "S":
                next_x = x
                next_y = y - 1

            next_pos = (next_x, next_y)
            if next_pos in visit:
                return True

            visit.add(next_pos)
            x = next_x
            y = next_y

        return False


def main():
    input = ("NES", "NESWW")

    for path in input:
        result = Solution().isPathCrossing(path)
        print(result)


if __name__ == "__main__":
    main()
