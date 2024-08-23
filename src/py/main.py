from __future__ import annotations


class Solution:
    def fractionAddition(self, expression):
        num = 0
        denom = 1

        i = 0
        while i < len(expression):
            curr_num = 0
            curr_denom = 0

            is_negative = False

            if expression[i] == "-" or expression[i] == "+":
                if expression[i] == "-":
                    is_negative = True
                i += 1

            while i < len(expression) and expression[i].isdigit():
                val = int(expression[i])
                curr_num = curr_num * 10 + val
                i += 1

            if is_negative:
                curr_num *= -1

            i += 1

            while i < len(expression) and expression[i].isdigit():
                val = int(expression[i])
                curr_denom = curr_denom * 10 + val
                i += 1

            num = num * curr_denom + curr_num * denom
            denom = denom * curr_denom

        gcd = abs(self.find_gcd(num, denom))

        num //= gcd
        denom //= gcd

        return f"{num}/{denom}"

    def find_gcd(self, a, b):
        if a == 0:
            return b
        return self.find_gcd(b % a, a)


def main():
    inputs: list[str] = ["-1/2+1/2", "-1/2+1/2+1/3", "1/3-1/2"]

    for input in inputs:
        result = Solution().fractionAddition(input)
        print(result)


if __name__ == "__main__":
    main()
