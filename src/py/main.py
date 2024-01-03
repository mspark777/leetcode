from __future__ import annotations
from typing import List


class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        result = 0
        prev_count = 0

        for row in bank:
            count = 0
            for device in row:
                if device == "1":
                    count += 1

            if count > 0:
                result += count * prev_count
                prev_count = count

        return result


def main():
    input = (["011001", "000000", "010100", "001000"], ["000", "111", "000"])

    for bank in input:
        result = Solution().numberOfBeams(bank)
        print(result)


if __name__ == "__main__":
    main()
