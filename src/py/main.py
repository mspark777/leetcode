from __future__ import annotations


class Solution:
    def checkPerfectNumber(self, num: int) -> bool:
        primes = [2, 3, 5, 7, 13, 17, 19, 31]

        for prime in primes:
            if self.euclid_euler(prime) == num:
                return True

        return False

    def euclid_euler(self, prime) -> int:
        return (2 ** (prime - 1)) * (2**prime - 1)


def main():
    input = (28, 7)

    for num in input:
        result = Solution().checkPerfectNumber(num)
        print(result)


if __name__ == "__main__":
    main()
