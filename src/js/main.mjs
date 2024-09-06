/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unnecessary-condition */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

class ListNode {
  constructor(val, next) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

/**
 * @param {number[]} nums
 * @param {ListNode | undefined} head
 * @return {ListNode | undefined}
 */
function modifiedList(nums, head) {
  const numSet = new Set(nums);

  while (head != null) {
    if (numSet.has(head.val)) {
      head = head.next;
    } else {
      break;
    }
  }

  if (head == null) {
    return undefined;
  }

  let curr = head;
  while (curr?.next != null) {
    if (numSet.has(curr.next.val)) {
      curr.next = curr.next.next;
    } else {
      curr = curr.next;
    }
  }

  return head;
}

function main() {
  const inputs = [
    [
      [0, 1, 1, 0],
      [0, 1, 1, 0],
      [0, 0, 0, 0],
    ],
    [[1, 1]],
  ];

  for (const input of inputs) {
    const result = minDays(input);
    console.log(result);
  }
}
main();
