class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function removeElements (head: ListNode | null, val: number): ListNode | null {
  const dummy = new ListNode(-1, head)
  let cur: ListNode | null = dummy

  while (cur != null) {
    while ((cur.next != null) && (cur.next.val === val)) {
      cur.next = cur.next.next
    }

    cur = cur.next
  }

  return dummy.next
}

interface Input {
  readonly head: ListNode | null
  readonly val: number
}

function arrtolist (nums: number[]): ListNode | null {
  const head = new ListNode()
  let tail = head
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return head.next
}

function listtoarr (node: ListNode | null): number[] {
  const nums: number[] = []
  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      val: 6,
      head: arrtolist([1, 2, 6, 3, 4, 5, 6])
    },
    {
      val: 1,
      head: arrtolist([])
    },
    {
      val: 7,
      head: arrtolist([7, 7, 7, 7])
    }
  ]

  for (const { head, val } of inputs) {
    const result = removeElements(head, val)
    console.log(listtoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
