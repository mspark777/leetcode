from __future__ import annotations


class Solution:
    def maxDepth(self, root) -> int:
        return self.inorder(root, 0)

    def inorder(self, node, depth) -> int:
        if node is None:
            return depth
        elif node.children is None:
            return depth + 1

        d = depth + 1
        for child in node.children:
            d = max(d, self.inorder(child, depth + 1))

        return d


def main():
    input = ([1, 2, 2, 4], [1, 1])

    for nums in input:
        result = Solution().findErrorNums(nums)
        print(result)


if __name__ == "__main__":
    main()
