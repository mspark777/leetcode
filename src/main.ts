class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function oddEvenList (head: ListNode | null): ListNode | null {
  if (head == null) {
    return null
  }

  let odd = head
  let even = head.next
  const evenHead = even
  while (even?.next != null) {
    odd.next = even.next
    odd = odd.next
    even.next = odd.next
    even = even.next
  }

  odd.next = evenHead
  return head
}

function arrtolist (nums: number[]): ListNode | null {
  const head = new ListNode()
  let tail = head
  for (const val of nums) {
    tail.next = new ListNode(val)
    tail = tail.next
  }

  return head.next
}

function listtoarr (node: ListNode | null): number[] {
  const nums = new Array<number>()
  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [1, 2, 3, 4, 5],
    [2, 1, 3, 5, 6, 4, 7]
  ]

  for (const nums of inputs) {
    const result = oddEvenList(arrtolist(nums))
    console.log(listtoarr(result))
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})
