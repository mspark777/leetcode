class Solution:
    def buddyStrings(self, s: str, goal: str) -> bool:
        slen = len(s)
        glen = len(goal)
        if slen != glen:
            return False

        if s == goal:
            counts: set[str] = set()
            for c in s:
                if c in counts:
                    return True
                else:
                    counts.add(c)

        first = -1
        second = -1
        for i in range(slen):
            c = s[i]
            g = goal[i]
            if c == g:
                continue

            if first < 0:
                first = i
            elif second < 0:
                second = i
            else:
                return False

        if first < 0:
            return False
        elif second < 0:
            return False

        if s[first] != goal[second]:
            return False
        elif s[second] != goal[first]:
            return False

        return True


def main():
    inputs = [("ab", "ba"), ("ab", "ab"), ("aa", "aa")]

    for s, goal in inputs:
        solution = Solution()
        result = solution.buddyStrings(s, goal)
        print(result)


if __name__ == "__main__":
    main()
