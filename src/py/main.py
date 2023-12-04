from __future__ import annotations


class Solution:
    def largestGoodInteger(self, num: str) -> str:
        result = -1

        for i in range(len(num) - 2):
            if num[i] == num[i + 1] == num[i + 2]:
                n = int(num[i])
                result = max(n, result)

        return str(result) * 3 if result >= 0 else ""


def main():
    inputs = ("6777133339", "2300019", "42352338")

    for num in inputs:
        result = Solution().largestGoodInteger(num)
        print(result)


if __name__ == "__main__":
    main()
