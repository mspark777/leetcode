class Solution:
    def predictPartyVictory(self, senate: str) -> str:
        R = "R"
        D = "D"
        rcount = 0
        dcount = 0
        rfloating = 0
        dfloating = 0

        queue = list(senate)
        for c in queue:
            if c == R:
                rcount += 1
            else:
                dcount += 1

        while (rcount > 0) and (dcount > 0):
            curr = queue.pop(0)

            if curr == D:
                if dfloating > 0:
                    dfloating -= 1
                    dcount -= 1
                else:
                    rfloating += 1
                    queue.append(D)
            else:
                if rfloating > 0:
                    rfloating -= 1
                    rcount -= 1
                else:
                    dfloating += 1
                    queue.append(R)

        return "Radiant" if rcount > 0 else "Dire"


def main():
    inputs = ["RD", "RDD"]

    for senate in inputs:
        solution = Solution()
        result = solution.predictPartyVictory(senate)
        print(result)


if __name__ == "__main__":
    main()
