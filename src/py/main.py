class Solution:
    def simplifyPath(self, path: str) -> str:
        segments = path.split("/")
        result: list[str] = []

        for seg in segments:
            if seg == "":
                continue
            elif seg == ".":
                continue
            elif seg == "..":
                if result:
                    result.pop()
            else:
                result.append(seg)

        return "/{}".format("/".join(result))


def main():
    inputs: list[str] = ["/home/", "/../", "/home//foo/"]

    for path in inputs:
        solution = Solution()
        result = solution.simplifyPath(path)
        print(result)


if __name__ == "__main__":
    main()
