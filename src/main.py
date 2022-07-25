"""
main
"""

from typing import Optional
from solution import Solution, ListNode

class Input:
    nums: list[int]
    pos: int
    def __init__(self, nums: list[int], pos: int):
        self.nums = nums
        self.pos = pos

def arrtolist(nums: list[int], pos: int) -> Optional[ListNode]:
    head = ListNode(0)
    tail = head
    cycle: Optional[ListNode] = None
    for i, n in enumerate(nums):
        node = ListNode(n)
        tail.next = node
        tail = node
        if i == pos:
            cycle = node

    tail.next = cycle
    return head.next


def main():
    inputs = [
            Input([3, 2, 0, -4], 1),
            Input([1, 2], 0),
            Input([1], -1),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.hasCycle(arrtolist(i.nums, i.pos))
        print(result)



if __name__ == "__main__":
    main()
