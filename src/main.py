from __future__ import annotations


class Solution:
    def restoreIpAddresses(self, s: str) -> list[str]:
        result: list[str] = []

        for len1 in range(max(1, len(s) - 9), min(4, len(s) - 2)):
            if not self.is_valid(s, 0, len1):
                continue

            for len2 in range(max(1, len(s) - (len1 + 6)), min(4, len(s) - (len1 + 1))):
                if not self.is_valid(s, len1, len2):
                    continue

                for len3 in range(
                    max(1, len(s) - (len1 + len2 + 3)),
                    min(4, len(s) - (len1 + len2)),
                ):
                    if self.is_valid(s, len1 + len2, len3) and self.is_valid(
                        s, len1 + len2 + len3, len(s) - (len1 + len2 + len3)
                    ):
                        result.append(
                            ".".join(
                                [
                                    s[0:len1],
                                    s[len1 : len1 + len2],
                                    s[len1 + len2 : len1 + len2 + len3],
                                    s[len1 + len2 + len3 :],
                                ]
                            )
                        )

        return result

    def is_valid(self, s: str, b: int, l: int) -> bool:
        return (l == 1) or (s[b] != "0" and (l < 3 or int(s[b : b + l]) <= 255))


def main():
    inputs = ["25525511135", "0000", "101023"]

    for s in inputs:
        solution = Solution()
        result = solution.restoreIpAddresses(s)
        print(result)


if __name__ == "__main__":
    main()
