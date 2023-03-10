from typing import Optional

from random import random
from lib import ListNode, new_list


class Solution:
    head: Optional[ListNode]

    def __init__(self, head: Optional[ListNode]):
        self.head = head

    def getRandom(self) -> int:
        scope = 1.0
        result = 0
        curr = self.head
        while curr:
            if (random() * scope) < 1.0:
                result = curr.val

            scope += 1.0
            curr = curr.next

        return result


def main():
    solution = Solution(new_list([1, 2, 3]))
    print(solution.getRandom())
    print(solution.getRandom())
    print(solution.getRandom())
    print(solution.getRandom())
    print(solution.getRandom())


if __name__ == "__main__":
    main()
