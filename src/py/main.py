from __future__ import annotations


class Solution:
    def countPrimes(self, n: int) -> int:
        if n <= 2:
            return 0

        dp = [i & 1 == 1 for i in range(n)]
        dp[1] = False
        dp[2] = True

        i = 3
        while True:
            j = i * i
            if j >= n:
                break

            if not dp[i]:
                i += 2
                continue

            k = j
            for k in range(j, n, i * 2):
                dp[k] = False

            i += 2

        result = 0
        for is_prime in dp[2:]:
            if is_prime:
                result += 1

        return result


def main():
    inputs = (10, 0, 1, 10000)

    for n in inputs:
        result = Solution().countPrimes(n)
        print(result)


if __name__ == "__main__":
    main()
