import "@total-typescript/ts-reset";

class _Node {
  val: number;
  prev: _Node | null;
  next: _Node | null;
  child: _Node | null;

  constructor(val?: number, prev?: _Node, next?: _Node, child?: _Node) {
    this.val = val === undefined ? 0 : val;
    this.prev = prev === undefined ? null : prev;
    this.next = next === undefined ? null : next;
    this.child = child === undefined ? null : child;
  }
}

function flatten(head: _Node | null): _Node | null {
  for (let curr = head; curr != null; curr = curr.next) {
    if (curr.child == null) {
      continue;
    }

    const next = curr.next;
    curr.next = curr.child;
    curr.next.prev = curr;
    curr.child = null;

    let child: _Node | null = curr.next;
    while (child.next != null) {
      child = child.next;
    }

    child.next = next;
    if (next != null) {
      next.prev = child;
    }
  }

  return head;
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 3, 6, 10, 12, 15],
    },
    {
      nums: [1, 2, 4, 7, 10],
    },
    {
      nums: [4, 4, 9, 10],
    },
  ];

  for (const input of inputs) {
    const result = averageValue(input.nums);
    console.log(result);
  }
}
main();
