from __future__ import annotations


class Solution:
    def toHex(self, num: int) -> str:
        if num == 0:
            return "0"

        if num < 0:
            num += 2**32

        hexes = (
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
        )

        result: list[str] = []
        while num != 0:
            result.append(hexes[num % 16])
            num //= 16
        result.reverse()

        return "".join(result)


def main():
    inputs = (26, -1)

    for num in inputs:
        result = Solution().toHex(num)
        print(result)


if __name__ == "__main__":
    main()
