from __future__ import annotations


class Solution:
    def convert(self, s: str, num_rows: int) -> str:
        if num_rows <= 1:
            return s

        last_row = num_rows - 1
        row = 0
        down = True
        result: list[list[str]] = [[] for i in range(num_rows)]

        for ch in s:
            result[row].append(ch)
            if row == last_row:
                down = False
            elif row == 0:
                down = True

            if down:
                row += 1
            else:
                row -= 1

        return "".join(["".join(arr) for arr in result])


def main():
    inputs: list[tuple[str, int]] = [
        ("PAYPALISHIRING", 3),
        ("PAYPALISHIRING", 4),
        ("A", 1),
        ("AB", 1),
    ]

    for s, num_rows in inputs:
        solution = Solution()
        result = solution.convert(s, num_rows)
        print(result)


if __name__ == "__main__":
    main()
