from __future__ import annotations


class Solution:
    def minFlipsMonoIncr(self, s: str) -> int:
        result = 0
        num = 0

        for c in s:
            if c == "0":
                result = min(num, result + 1)
            else:
                num += 1

        return result


def main():
    inputs: list[str] = ["00110", "010110", "00011000"]

    for s in inputs:
        solution = Solution()
        result = solution.minFlipsMonoIncr(s)
        print(result)


if __name__ == "__main__":
    main()
