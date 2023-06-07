class Solution:
    def minFlips(self, a: int, b: int, c: int) -> int:
        d = (a | b) ^ c
        e = a & b & d
        return d.bit_count() + e.bit_count()


def main():
    inputs = [(2, 6, 5), (4, 2, 7), (1, 2, 3), (7, 3, 9)]

    for a, b, c in inputs:
        solution = Solution()
        result = solution.minFlips(a, b, c)
        print(result)


if __name__ == "__main__":
    main()
