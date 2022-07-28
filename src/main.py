"""
main
"""

from typing import Optional
from solution import Solution, ListNode


class Input:
    headA: Optional[ListNode]
    headB: Optional[ListNode]
    def __init__(self, headA: Optional[ListNode], headB: Optional[ListNode]):
        self.headA = headA
        self.headB = headB

def arrtolist(nums: list[int]) -> Optional[ListNode]:
    head = ListNode(0, None)
    tail = head
    for n in nums:
        tail.next = ListNode(n, None)
        tail = tail.next

    return head.next

def createinput(numsa: list[int], numsb: list[int], skipa: int, skipb: int) -> Input:
    heada = arrtolist(numsa[0:skipa])
    headb = arrtolist(numsb[0:skipb])
    if heada is None or headb is None:
        return Input(heada, headb)

    remain = arrtolist(numsa[skipa:])

    tail = heada
    while tail is not None and tail.next is not None:
        tail = tail.next
    tail.next = remain

    tail = headb
    while tail is not None and tail.next is not None:
        tail = tail.next
    tail.next = remain

    return Input(heada, headb)


def listtoarr(node: Optional[ListNode]) -> list[int]:
    nums = []
    while node is not None:
        nums.append(node.val)
        node = node.next

    return nums

def main():
    inputs = [
      createinput([4, 1, 8, 4, 5], [5, 6, 1, 8, 4, 5], 2, 3),
      createinput([1, 9, 1, 2, 4], [3, 2, 4], 3, 1),
      createinput([2, 6, 4], [1, 5], 3, 2)
    ]

    sol = Solution()
    for i in inputs:
        result = sol.getIntersectionNode(i.headA, i.headB)
        print(listtoarr(result))



if __name__ == "__main__":
    main()
