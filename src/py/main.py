from typing import Optional

from utils import ListNode, new_list, list_to_arr


class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        elif head.next is None:
            return head

        dummy = ListNode(-1, None)
        prev = dummy
        curr = head

        while (curr is not None) and (curr.next is not None):
            prev.next = curr.next
            curr.next = prev.next.next
            prev.next.next = curr

            prev = curr
            curr = curr.next

        return dummy.next


def main():
    inputs = [new_list([1, 2, 3, 4]), new_list([]), new_list([1])]

    for head in inputs:
        solution = Solution()
        result = solution.swapPairs(head)
        print(list_to_arr(result))


if __name__ == "__main__":
    main()
