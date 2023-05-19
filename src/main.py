from typing import List


class Solution:
    def isBipartite(self, graph: List[List[int]]) -> bool:
        NONE = 0
        RED = 1
        # BLUE = -1

        colors = [NONE for _ in range(len(graph))]
        stack: list[int] = []
        for i in range(len(graph)):
            if colors[i] != NONE:
                continue

            colors[i] = RED
            stack.append(i)
            while stack:
                vertex = stack.pop()
                color = colors[vertex]
                for adjacent in graph[vertex]:
                    acolor = colors[adjacent]
                    if acolor == NONE:
                        colors[adjacent] = -color
                        stack.append(adjacent)
                    elif color == acolor:
                        return False

        return True


def main():
    inputs = [[[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]], [[1, 3], [0, 2], [1, 3], [0, 2]]]

    for graph in inputs:
        solution = Solution()
        result = solution.isBipartite(graph)
        print(result)


if __name__ == "__main__":
    main()
