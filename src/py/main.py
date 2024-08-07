from __future__ import annotations

number_to_words_map = {
    1000000000: "Billion",
    1000000: "Million",
    1000: "Thousand",
    100: "Hundred",
    90: "Ninety",
    80: "Eighty",
    70: "Seventy",
    60: "Sixty",
    50: "Fifty",
    40: "Forty",
    30: "Thirty",
    20: "Twenty",
    19: "Nineteen",
    18: "Eighteen",
    17: "Seventeen",
    16: "Sixteen",
    15: "Fifteen",
    14: "Fourteen",
    13: "Thirteen",
    12: "Twelve",
    11: "Eleven",
    10: "Ten",
    9: "Nine",
    8: "Eight",
    7: "Seven",
    6: "Six",
    5: "Five",
    4: "Four",
    3: "Three",
    2: "Two",
    1: "One",
}


class Solution:
    def numberToWords(self, num: int) -> str:
        if num == 0:
            return "Zero"

        for value, word in number_to_words_map.items():
            if num >= value:
                prefix = (self.numberToWords(num // value) + " ") if num >= 100 else ""
                unit = word
                suffix = (
                    "" if num % value == 0 else " " + self.numberToWords(num % value)
                )
                return prefix + unit + suffix
        return ""


def main():
    inputs: list[int] = [123, 12345, 1234567]

    for input in inputs:
        result = Solution().numberToWords(input)
        print(result)


if __name__ == "__main__":
    main()
