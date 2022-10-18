from __future__ import annotations
from typing import Optional, List


class Solution:
    def countAndSay(self, n: int) -> str:
        result = ["1"]
        while n > 1:
            n -= 1
            temp: list[str] = []
            count = 1
            ch = result[0]
            for j in range(1, len(result)):
                c = result[j]
                if ch == c:
                    count += 1
                else:
                    temp.append(f"{count}")
                    temp.append(ch)

                    ch = c
                    count = 1
            temp.append(f"{count}")
            temp.append(ch)
            result = temp

        return "".join(result)


def main():
    inputs: list[int] = [1, 4]

    solution = Solution()
    for n in inputs:
        result = solution.countAndSay(n)
        print(result)


if __name__ == "__main__":
    main()
