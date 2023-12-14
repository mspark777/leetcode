from __future__ import annotations


class Solution:
    def fractionToDecimal(self, numerator: int, denominator: int) -> str:
        if numerator == 0:
            return "0"

        result: list[str] = []
        if numerator > 0 and denominator < 0:
            result.append("-")
        elif numerator < 0 and denominator > 0:
            result.append("-")

        numerator = abs(numerator)
        denominator = abs(denominator)

        result.append(str(numerator // denominator))
        numerator %= denominator
        if numerator == 0:
            return "".join(result)

        result.append(".")
        memo = dict[int, int]()
        memo[numerator] = len(result)
        while numerator != 0:
            numerator *= 10
            result.append(str(numerator // denominator))
            numerator %= denominator

            if numerator in memo:
                idx = memo[numerator]
                result.insert(idx, "(")
                result.append(")")
                break
            else:
                memo[numerator] = len(result)

        return "".join(result)


def main():
    inputs = ((1, 2), (2, 1), (4, 333))

    for numerator, denominator in inputs:
        result = Solution().fractionToDecimal(numerator, denominator)
        print(result)


if __name__ == "__main__":
    main()
