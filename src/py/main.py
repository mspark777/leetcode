from typing import List, Optional

from utils import ListNode, new_list, list_to_arr


class Solution:
    def swapNodes(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        left = head
        for i in range(1, k):
            left = left.next if left is not None else None

        if left is None:
            return head

        right = head
        i = left
        while i.next is not None:
            i = i.next
            right = right.next if right is not None else None

        if right is None:
            return head

        l = left.val
        r = right.val
        left.val = r
        right.val = l

        return head


def main():
    inputs = [
        (new_list([1, 2, 3, 4, 5]), 2),
        (new_list([7, 9, 6, 6, 7, 8, 3, 0, 9, 5]), 5),
    ]

    for head, k in inputs:
        solution = Solution()
        result = solution.swapNodes(head, k)
        print(list_to_arr(result))


if __name__ == "__main__":
    main()
