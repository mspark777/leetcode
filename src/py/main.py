from __future__ import annotations
from typing import Optional
from list_node import ListNode, atol, ltoa


class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        elif head.next is None:
            return head

        slow: Optional[ListNode] = head
        fast: Optional[ListNode] = head.next

        while slow is not None and fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next

        head2: Optional[ListNode] = None
        if slow is not None:
            head2 = slow.next
            slow.next = None

        first = self.sortList(head)
        second = self.sortList(head2)

        return self.merge(first, second)

    def merge(
        self, left: Optional[ListNode], right: Optional[ListNode]
    ) -> Optional[ListNode]:
        if left is None:
            return right
        elif right is None:
            return left

        head = left if left.val <= right.val else right
        l: Optional[ListNode] = left
        r: Optional[ListNode] = right
        if head == left:
            l = l.next
        else:
            r = r.next

        h = head

        while l is not None and r is not None:
            if l.val <= r.val:
                h.next = l
                h = l
                l = l.next
            else:
                h.next = r
                h = r
                r = r.next

        if l is not None:
            h.next = l
        elif r is not None:
            h.next = r

        return head


def main():
    inputs = ([4, 2, 1, 3], [-1, 5, 3, 4, 0])

    for nums in inputs:
        head = atol(nums)
        result = Solution().sortList(head)
        print(ltoa(result))


if __name__ == "__main__":
    main()
