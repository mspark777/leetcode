from typing import List


class Solution:
    def nextGreatestLetter(self, letters: List[str], target: str) -> str:
        t = ord(target)
        left = 0
        right = len(letters) - 1
        while left <= right:
            middle = (left + right) // 2
            letter = ord(letters[middle])
            if letter <= t:
                left = middle + 1
            else:
                right = middle - 1

        return letters[left] if left < len(letters) else letters[0]


def main():
    inputs = [
        (["c", "f", "j"], "a"),
        (["c", "f", "j"], "c"),
        (["x", "x", "y", "y"], "z"),
    ]

    for letters, target in inputs:
        solution = Solution()
        result = solution.nextGreatestLetter(letters, target)
        print(result)


if __name__ == "__main__":
    main()
