from __future__ import annotations


class Solution:
    def addStrings(self, num1: str, num2: str) -> str:
        ZERO = ord("0")
        carry = 0
        pos1 = len(num1) - 1
        pos2 = len(num2) - 1
        result: list[str] = []
        while pos1 >= 0 or pos2 >= 0 or carry > 0:
            s1 = num1[pos1] if pos1 >= 0 else "0"
            s2 = num2[pos2] if pos2 >= 0 else "0"
            n1 = ord(s1) - ZERO
            n2 = ord(s2) - ZERO
            s = n1 + n2 + carry
            carry = s // 10
            result.append(chr(s % 10 + ZERO))

            pos1 -= 1
            pos2 -= 1

        result.reverse()
        return "".join(result)


def main():
    input = (("11", "123"), ("456", "77"), ("0", "0"))

    for num1, num2 in input:
        result = Solution().addStrings(num1, num2)
        print(result)


if __name__ == "__main__":
    main()
