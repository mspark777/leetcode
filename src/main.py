from __future__ import annotations


class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        words = s.split(" ")
        patterns = [ch for ch in pattern]
        if len(words) != len(patterns):
            return False

        ptow: dict[str, str] = {}
        wtop: dict[str, str] = {}

        for i in range(len(words)):
            word = words[i]
            p = patterns[i]
            if p in ptow:
                if ptow[p] != word:
                    return False
            else:
                ptow[p] = word

            if word in wtop:
                if wtop[word] != p:
                    return False
            else:
                wtop[word] = p

        return True


def main():
    inputs: list[tuple[str, str]] = [
        ("abba", "dog cat cat dog"),
        ("abba", "dog cat cat fish"),
        ("aaaa", "dog cat cat dog"),
        ("abba", "dog dog dog dog"),
    ]

    solution = Solution()
    for p, s in inputs:
        result = solution.wordPattern(p, s)
        print(result)


if __name__ == "__main__":
    main()
