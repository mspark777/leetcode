class Solution:
    def removeStars(self, s: str) -> str:
        stack: list[str] = []
        for ch in s:
            if ch == "*":
                stack.pop()
            else:
                stack.append(ch)

        return "".join(stack)


def main():
    inputs: list[str] = ["leet**cod*e", "erase*****"]

    for s in inputs:
        solution = Solution()
        result = solution.removeStars(s)
        print(result)


if __name__ == "__main__":
    main()
