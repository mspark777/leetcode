class Solution:
    def isValid(self, s: str) -> bool:
        stack: list[str] = []
        for ch in s:
            if ch == "(":
                stack.append(")")
            elif ch == "{":
                stack.append("}")
            elif ch == "[":
                stack.append("]")
            elif not stack:
                return False
            elif stack.pop() != ch:
                return False

        return not stack


def main():
    inputs: list[str] = ["()", "()[]{}", "(]"]

    for s in inputs:
        solution = Solution()
        result = solution.isValid(s)
        print(result)


if __name__ == "__main__":
    main()
