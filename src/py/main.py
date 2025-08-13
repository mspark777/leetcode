from collections import Counter


class Solution:
    def countLargestGroup(self, n: int) -> int:
        counts = Counter[int]()
        for i in range(1, n + 1):
            key = sum([int(x) for x in str(i)])
            counts[key] += 1
        max_count = max(counts.values())
        return sum(1 for v in counts.values() if v == max_count)


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs = [Input(13), Input(2)]

    for input in inputs:
        result = Solution().countLargestGroup(input.n)
        print(result)


if __name__ == "__main__":
    main()
