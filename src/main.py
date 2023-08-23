from __future__ import annotations
from queue import PriorityQueue
from collections import Counter


class Solution:
    def reorganizeString(self, s: str) -> str:
        queue: PriorityQueue[tuple[int, str]] = PriorityQueue()

        for ch, count in Counter(s).items():
            queue.put((-count, ch))

        result: list[str] = []
        while not queue.empty():
            count0, ch0 = queue.get()
            if not result or (ch0 != result[-1]):
                result.append(ch0)
                count0 += 1
                if count0 < 0:
                    queue.put((count0, ch0))
            else:
                if queue.empty():
                    return ""

                count1, ch1 = queue.get()
                result.append(ch1)
                count1 += 1
                if count1 < 0:
                    queue.put((count1, ch1))
                queue.put((count0, ch0))

        return "".join(result)


def main():
    inputs = ["aab", "aaab"]

    for s in inputs:
        solution = Solution()
        result = solution.reorganizeString(s)
        print(result)


if __name__ == "__main__":
    main()
