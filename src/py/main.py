from __future__ import annotations


class Solution:
    def countHomogenous(self, s: str) -> int:
        result = 0
        curr = 0
        MOD = 10**9 + 7

        for i in range(len(s)):
            if i == 0 or s[i] == s[i - 1]:
                curr += 1
            else:
                curr = 1

            result = (result + curr) % MOD

        return result


def main():
    inputs = ("abbcccaa", "xy", "zzzzz")

    for s in inputs:
        result = Solution().countHomogenous(s)
        print(result)


if __name__ == "__main__":
    main()
