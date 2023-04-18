class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        result: list[str] = []

        for i in range(max(len(word1), len(word2))):
            if i < len(word1):
                result.append(word1[i])

            if i < len(word2):
                result.append(word2[i])

        return "".join(result)


def main():
    inputs: list[tuple[str, str]] = [("abc", "pqr"), ("ab", "pqrs"), ("abcd", "pq")]

    for word1, word2 in inputs:
        solution = Solution()
        result = solution.mergeAlternately(word1, word2)
        print(result)


if __name__ == "__main__":
    main()
