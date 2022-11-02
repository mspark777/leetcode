from __future__ import annotations
from typing import Optional, List
from collections import deque


class Solution:
    def minMutation(self, start: str, end: str, bank: List[str]) -> int:
        queue = deque([start])
        seens = set([start])
        bank_set = set(bank)

        result = 0

        while queue:
            queue_len = len(queue)
            for i in range(queue_len):
                gene = queue.popleft()
                if gene == end:
                    return result

                for g in "ACGT":
                    for j in range(len(gene)):
                        neighbor = gene[:j] + g + gene[j + 1 :]
                        if neighbor not in seens and neighbor in bank_set:
                            queue.append(neighbor)
                            seens.add(neighbor)
            result += 1

        return -1


class Input:
    start: str
    end: str
    bank: list[str]

    def __init__(self, start: str, end: str, bank: list[str]) -> None:
        self.start = start
        self.end = end
        self.bank = bank


def main():
    inputs: list[Input] = [
        Input("AACCGGTT", "AACCGGTA", ["AACCGGTA"]),
        Input("AACCGGTT", "AAACGGTA", ["AACCGGTA", "AACCGCTA", "AAACGGTA"]),
        Input("AAAAACCC", "AACCCCCC", ["AAAACCCC", "AAACCCCC", "AACCCCCC"]),
        Input("AACCGGTT", "AACCGCTA", ["AACCGGTA", "AACCGCTA", "AAACGGTA"]),
    ]

    solution = Solution()
    for input in inputs:
        start = input.start
        end = input.end
        bank = input.bank
        result = solution.minMutation(start, end, bank)
        print(result)


if __name__ == "__main__":
    main()
