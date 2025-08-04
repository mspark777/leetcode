import "@total-typescript/ts-reset";

class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function getDecimalValue(head: ListNode | null): number {
  let result = 0;

  for (let node = head; node != null; node = node.next) {
    result = (result << 1) | node.val;
  }

  return result;
}

function tolist(nums: number[] | null): ListNode | null {
  if (nums == null) {
    return null;
  }

  const dummy = new ListNode();
  let tail = dummy;
  for (const n of nums) {
    tail.next = new ListNode(n);
    tail = tail.next;
  }

  return dummy.next;
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 0, 1],
    },
    {
      nums: [0],
    },
  ];

  for (const input of inputs) {
    const result = getDecimalValue(tolist(input.nums));
    console.log(result);
  }
}
main();
