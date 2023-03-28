from typing import List


class Solution:
    def mincostTickets(self, days: List[int], costs: List[int]) -> int:
        memos = [0] * len(days)
        durations = [1, 7, 30]

        return self.dp(days, costs, memos, durations, 0)

    def dp(
        self,
        days: list[int],
        costs: list[int],
        memos: list[int],
        durations: list[int],
        i: int,
    ) -> int:
        if i >= len(days):
            return 0
        elif memos[i] != 0:
            return memos[i]

        result = 2**31
        j = i
        for duration, cost in zip(durations, costs):
            while j < len(days):
                k = days[i] + duration
                if days[j] < k:
                    j += 1
                else:
                    break
            recv = self.dp(days, costs, memos, durations, j)
            result = min(result, recv + cost)

        memos[i] = result
        return result


def main():
    inputs: list[list[list[int]]] = [
        [[1, 4, 6, 7, 8, 20], [2, 7, 15]],
        [[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15]],
    ]

    for [days, costs] in inputs:
        solution = Solution()
        result = solution.mincostTickets(days, costs)
        print(result)


if __name__ == "__main__":
    main()
