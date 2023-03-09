from typing import List, Optional

from lib import ListNode, new_cycle_list


class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast = head
        slow = head

        while (fast is not None) and (fast.next is not None):
            fast = fast.next.next
            slow = slow.next if slow is not None else None

            if fast == slow:
                break

        if (fast is None) or (fast.next is None):
            return None

        fast = head

        while fast != slow:
            fast = fast.next if fast is not None else None
            slow = slow.next if slow is not None else None

        return fast


def main():
    inputs: list[ListNode] = [
        new_cycle_list([3, 2, 0, -4], 1),
        new_cycle_list([1, 2], 0),
        new_cycle_list([1], -1),
    ]

    for head in inputs:
        solution = Solution()
        result = solution.detectCycle(head)
        print(result.val if result is not None else None)


if __name__ == "__main__":
    main()
