from __future__ import annotations
from typing import List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        frequencies = [0] * 26
        max_count = 0

        for task in tasks:
            i = ord(task) - ord("A")
            frequencies[i] += 1
            max_count = max(max_count, frequencies[i])

        time = (max_count - 1) * (n + 1)

        for f in frequencies:
            if f == max_count:
                time += 1

        return max(len(tasks), time)


def main():
    input = [
        (["A", "A", "A", "B", "B", "B"], 2),
        (["A", "C", "A", "B", "D", "B"], 1),
        (["A", "A", "A", "B", "B", "B"], 3),
    ]

    for tasks, n in input:
        result = Solution().leastInterval(tasks, n)
        print(result)


if __name__ == "__main__":
    main()
