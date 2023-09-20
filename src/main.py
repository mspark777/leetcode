from __future__ import annotations
from typing import Optional, List


class ListNode:
    val: int
    next: Optional[ListNode]

    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(
        self,
        val: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def sortedListToBST(self, head: Optional[ListNode]) -> Optional[TreeNode]:
        if head is None:
            return None
        elif head.next is None:
            return TreeNode(head.val)

        slow = head
        fast = head
        prev = head

        while (fast is not None) and (fast.next is not None) and (slow is not None):
            fast = fast.next.next
            prev = slow
            slow = slow.next

        prev.next = None

        if slow is None:
            return None

        node = TreeNode(
            slow.val, self.sortedListToBST(head), self.sortedListToBST(slow.next)
        )

        return node


def arrtolist(nums: list[int]) -> Optional[ListNode]:
    dummy = ListNode()
    tail = dummy
    for num in nums:
        next = ListNode(num)
        tail.next = next
        tail = next

    return dummy.next


def treetoarr(node: Optional[TreeNode]) -> list[int]:
    nums: list[int] = []

    def travel(n: Optional[TreeNode], l: list[int]):
        if n is None:
            return

        if n.left is not None:
            travel(n.left, l)

        l.append(n.val)

        if n.right is not None:
            travel(n.right, l)

    travel(node, nums)
    return nums


def main():
    inputs = [[-10, -3, 0, 5, 9], []]

    for head in inputs:
        solution = Solution()
        result = solution.sortedListToBST(arrtolist(head))
        print(treetoarr(result))


if __name__ == "__main__":
    main()
