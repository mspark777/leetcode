import '@total-typescript/ts-reset'

class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function partition (head: ListNode | null, x: number): ListNode | null {
  const beforeHead = new ListNode(0)
  const afterHead = new ListNode(0)
  let before = beforeHead
  let after = afterHead

  while (head != null) {
    const { val } = head
    if (head.val < x) {
      before.next = new ListNode(val)
      before = before.next
    } else {
      after.next = new ListNode(val)
      after = after.next
    }

    head = head.next
  }
  after.next = null
  before.next = afterHead.next

  return beforeHead.next
}

interface Input {
  readonly head: number[]
  readonly x: number
}

function arrtolist (vals: number[]): ListNode | null {
  const dummy = new ListNode()
  let tail = dummy
  for (const val of vals) {
    tail.next = new ListNode(val)
    tail = tail.next
  }
  return dummy.next
}

function listtoarr (node: ListNode | null): number[] {
  const vals: number[] = []
  while (node != null) {
    vals.push(node.val)
    node = node.next
  }

  return vals
}

function main (): void {
  const inputs: Input[] = [
    { head: [1, 4, 3, 2, 5, 2], x: 3 },
    { head: [2, 1], x: 2 }
  ]

  for (const { head, x } of inputs) {
    const result = partition(arrtolist(head), x)
    console.log(listtoarr(result))
  }
}
main()
