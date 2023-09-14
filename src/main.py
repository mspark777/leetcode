from __future__ import annotations
from collections import defaultdict


class Solution:
    def findItinerary(self, tickets: list[list[str]]) -> list[str]:
        graph: dict[str, list[str]] = defaultdict(list)

        for src, dst in sorted(tickets, reverse=True):
            graph[src].append(dst)

        stack: list[str] = ["JFK"]
        itinerary: list[str] = []

        while stack:
            while graph[stack[-1]]:
                stack.append(graph[stack[-1]].pop())
            itinerary.append(stack.pop())

        return itinerary[::-1]


def main():
    inputs = [
        [["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]],
        [
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"],
        ],
    ]

    for tickets in inputs:
        solution = Solution()
        result = solution.findItinerary(tickets)
        print(result)


if __name__ == "__main__":
    main()
