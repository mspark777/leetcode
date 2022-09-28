
class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function arrtolist (nums: number[]): ListNode | null {
  const dummy = new ListNode()
  let tail = dummy
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return dummy.next
}

function listtoarr (node: ListNode | null): number[] {
  const nums: number[] = []
  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

function removeNthFromEnd (head: ListNode | null, n: number): ListNode | null {
  if (head == null) {
    return null
  }

  let right: ListNode | null = head
  for (let i = 0; i < n; i += 1) {
    right = right?.next ?? null
  }

  if (right == null) {
    return head.next
  }

  let left: ListNode | null = head
  while (right.next != null) {
    right = right.next
    left = left?.next ?? null
  }

  if (left != null) {
    left.next = left?.next?.next ?? null
  }

  return head
}

interface Input {
  readonly head: ListNode | null
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      head: arrtolist([1, 2, 3, 4, 5]),
      n: 2
    }, {
      head: arrtolist([1]),
      n: 1
    }, {
      head: arrtolist([1, 2]),
      n: 1
    }
  ]

  for (const { head, n } of inputs) {
    const result = removeNthFromEnd(head, n)
    console.log(listtoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
