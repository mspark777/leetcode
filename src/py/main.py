from typing import Optional

from utils import ListNode, new_list


class Solution:
    def pairSum(self, head: Optional[ListNode]) -> int:
        node_count = 0
        node = head
        while node is not None:
            node_count += 1
            node = node.next

        nums = [0 for i in range(node_count)]
        node = head
        i = 0
        while node is not None:
            nums[i] = node.val
            node = node.next
            i += 1

        i = 0
        j = node_count - 1
        result = 0

        while i < j:
            result = max(result, nums[i] + nums[j])
            i += 1
            j -= 1

        return result


def main():
    inputs = [new_list([5, 4, 2, 1]), new_list([4, 2, 2, 3]), new_list([1, 100000])]

    for head in inputs:
        solution = Solution()
        result = solution.pairSum(head)
        print(result)


if __name__ == "__main__":
    main()
