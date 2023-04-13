from typing import List


class Solution:
    def validateStackSequences(self, pushed: List[int], popped: List[int]) -> bool:
        num_count = len(pushed)
        stack: list[int] = []

        pop_count = 0
        for p in pushed:
            stack.append(p)
            while pop_count < num_count:
                if not stack:
                    break
                elif stack[-1] != popped[pop_count]:
                    break
                else:
                    stack.pop()
                    pop_count += 1

        return pop_count == num_count


def main():
    inputs: list[tuple[list[int], list[int]]] = [
        ([1, 2, 3, 4, 5], [4, 5, 3, 2, 1]),
        ([1, 2, 3, 4, 5], [4, 3, 5, 1, 2]),
    ]

    for pushed, popped in inputs:
        solution = Solution()
        result = solution.validateStackSequences(pushed, popped)
        print(result)


if __name__ == "__main__":
    main()
