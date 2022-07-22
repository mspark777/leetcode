"""
main
"""

from typing import Optional
from solution import Solution, ListNode

def arrtolist(arr: list[int]) -> Optional[ListNode]:
    head = ListNode()
    tail = head

    for n in arr:
        tail.next = ListNode(n)
        tail = tail.next

    return head.next

def listtoarr(node: Optional[ListNode]) -> list[int]:
    arr: list[int] = []
    while node is not None:
        arr.append(node.val)
        node = node.next

    return arr

class Input:
    def __init__(self, head: list[int], x: int):
        self.head = head
        self.x = x

def main():
    inputs = [
            Input([1, 4, 3, 2, 5, 2], 3),
            Input([2, 1], 2),
            Input([], 1),
    ]
    sol = Solution()
    for i in inputs:
        result = sol.partition(arrtolist(i.head), i.x)
        print(listtoarr(result))



if __name__ == "__main__":
    main()
