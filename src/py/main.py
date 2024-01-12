from __future__ import annotations


class Solution:
    def halvesAreAlike(self, s: str) -> bool:
        n = len(s)
        halves = n // 2
        a = s[0:halves]
        b = s[halves:n]

        vowels = set[str](["a", "e", "i", "o", "u"])
        acount = 0
        bcount = 0
        for i in range(halves):
            if a[i].lower() in vowels:
                acount += 1

            if b[i].lower() in vowels:
                bcount += 1

        return acount == bcount


def main():
    input = ("book", "textbook")

    for s in input:
        result = Solution().halvesAreAlike(s)
        print(result)


if __name__ == "__main__":
    main()
