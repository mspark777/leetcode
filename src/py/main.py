from __future__ import annotations


class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        frequency1 = [0] * 26
        for ch in word1:
            i = ord(ch) - ord("a")
            frequency1[i] += 1

        frequency2 = [0] * 26
        for ch in word2:
            i = ord(ch) - ord("a")
            frequency2[i] += 1

        for freq1, freq2 in zip(frequency1, frequency2):
            if freq1 == 0 and freq2 != 0:
                return False
            elif freq1 != 0 and freq2 == 0:
                return False

        frequency1.sort()
        frequency2.sort()
        for freq1, freq2 in zip(frequency1, frequency2):
            if freq1 != freq2:
                return False

        return True


def main():
    input = (("abc", "bca"), ("a", "aa"), ("cabbba", "abbccc"))

    for word1, word2 in input:
        result = Solution().closeStrings(word1, word2)
        print(result)


if __name__ == "__main__":
    main()
