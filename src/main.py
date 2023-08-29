from __future__ import annotations


class Solution:
    def bestClosingTime(self, customers: str) -> int:
        INCOME = "Y"
        cur_penalty = customers.count(INCOME)
        min_penalty = cur_penalty
        result = 0

        for i, ch in enumerate(customers):
            if ch == INCOME:
                cur_penalty -= 1
            else:
                cur_penalty += 1

            if cur_penalty < min_penalty:
                min_penalty = cur_penalty
                result = i + 1

        return result


def main():
    inputs = ["YYNY", "NNNNN", "YYYY"]

    for customers in inputs:
        solution = Solution()
        result = solution.bestClosingTime(customers)
        print(result)


if __name__ == "__main__":
    main()
