from __future__ import annotations


class Solution:
    def licenseKeyFormatting(self, s: str, k: int) -> str:
        chars = "".join(s.split("-"))
        char_count = len(chars)
        first_len = char_count % k
        if first_len == 0:
            first_len = k

        result = [chars[0:first_len].upper()]

        begin = first_len
        end = begin + k
        while end <= char_count:
            result.append(chars[begin:end].upper())
            begin = end
            end += k

        return "-".join(result)


def main():
    input = (("5F3Z-2e-9-w", 4), ("2-5g-3-J", 2), ("r", 1))

    for s, k in input:
        result = Solution().licenseKeyFormatting(s, k)
        print(result)


if __name__ == "__main__":
    main()
