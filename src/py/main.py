class Solution:
    def partitionString(self, s: str) -> int:
        seens = [-1] * 26
        count = 1
        substart = 0

        ACODE = ord("a")
        for i, ch in enumerate(s):
            code = ord(ch) - ACODE
            if seens[code] >= substart:
                count += 1
                substart = i
            seens[code] = i

        return count


def main():
    inputs: list[str] = ["abacaba", "ssssss"]

    for s in inputs:
        solution = Solution()
        result = solution.partitionString(s)
        print(result)


if __name__ == "__main__":
    main()
