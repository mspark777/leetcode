from __future__ import annotations


class Solution:
    def nearestPalindromic(self, n: str) -> str:
        num = int(n)
        previous = self.previous_palidrome(num)
        next = self.next_palidrome(num)
        if abs(previous - num) <= abs(next - num):
            return str(previous)
        return str(next)

    def convert(self, num: int) -> int:
        s = str(num)
        n = len(s)
        l = (n - 1) // 2
        r = n // 2
        s_list = list(s)

        while l >= 0:
            s_list[r] = s_list[l]
            r += 1
            l -= 1
        return int("".join(s_list))

    def previous_palidrome(self, num: int) -> int:
        left = 0
        right = num
        result = 0

        while left <= right:
            mid = (left + right) // 2
            palin = self.convert(mid)
            if palin < num:
                result = palin
                left = mid + 1
            else:
                right = mid - 1

        return result

    def next_palidrome(self, num: int) -> int:
        left = num
        right = int(1e18)
        result = 0
        while left <= right:
            mid = (left + right) // 2
            palin = self.convert(mid)
            if palin > num:
                result = palin
                right = mid - 1
            else:
                left = mid + 1
        return result


def main():
    inputs: list[str] = ["123", "1"]

    for input in inputs:
        result = Solution().nearestPalindromic(input)
        print(result)


if __name__ == "__main__":
    main()
