class Solution:
    def lengthAfterTransformations(self, s: str, t: int) -> int:
        mod = 1000000007
        counts = [0] * 26
        for ch in s:
            counts[ord(ch) - ord("a")] += 1

        for _ in range(t):
            next = [0] * 26
            next[0] = counts[25]
            next[1] = (counts[25] + counts[0]) % mod
            for i in range(2, 26):
                next[i] = counts[i - 1]
            counts = next

        result = sum(counts) % mod
        return result


class Input:
    s: str
    t: int

    def __init__(self, s: str, t: int):
        self.s = s
        self.t = t


def main():
    inputs = [Input("abcyy", 2), Input("azbk", 1)]

    for input in inputs:
        result = Solution().lengthAfterTransformations(input.s, input.t)
        print(result)


if __name__ == "__main__":
    main()
