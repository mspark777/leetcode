from __future__ import annotations


class Solution:
    def reverseWords(self, s: str) -> str:
        result: list[str] = []
        for s in s.split(" "):
            if s == "":
                continue

            result.append(s)

        result.reverse()
        return " ".join(result)


def main():
    inputs = ("the sky is blue", "  hello world  ", "a good   example")

    for s in inputs:
        result = Solution().reverseWords(s)
        print(result)


if __name__ == "__main__":
    main()
