from __future__ import annotations


class Solution:
    def customSortString(self, order: str, s: str) -> str:
        orders = list(order)
        chars = list(s)
        chars.sort(key=lambda x: orders.index(x) if x in orders else len(orders))

        return "".join(chars)


def main():
    input = [("cba", "abcd"), ("bcafg", "abcd")]

    for order, s in input:
        result = Solution().customSortString(order, s)
        print(result)


if __name__ == "__main__":
    main()
