import '@total-typescript/ts-reset'

class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function ltoa (node: ListNode | null): number[] {
  const arr: number[] = []
  while (node != null) {
    arr.push(node.val)
    node = node.next
  }

  return arr
}

function isNotEmpty (arr: number[]): boolean {
  return arr.length > 0
}

function addTwoNumbers (l1: ListNode | null, l2: ListNode | null): ListNode | null {
  const stack1 = ltoa(l1)
  const stack2 = ltoa(l2)

  let total = 0n
  let carry = 0n
  let result = new ListNode()
  while (isNotEmpty(stack1) || isNotEmpty(stack2)) {
    if (isNotEmpty(stack1)) {
      total += BigInt(stack1.pop() as number)
    }

    if (isNotEmpty(stack2)) {
      total += BigInt(stack2.pop() as number)
    }

    result.val = Number(total % 10n)
    carry = total / 10n
    const head = new ListNode(Number(carry))
    head.next = result
    result = head
    total = carry
  }

  return carry === 0n ? result.next : result
}

function atol (arr: number[]): ListNode | null {
  const head = new ListNode()
  let tail = head
  for (const num of arr) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return head.next
}

function main (): void {
  const inputs = [
    [atol([7, 2, 4, 3]), atol([5, 6, 4])],
    [atol([2, 4, 3]), atol([5, 6, 4])],
    [atol([0]), atol([0])]
  ]

  for (const [l1, l2] of inputs) {
    const result = addTwoNumbers(l1 as any, l2 as any)
    console.log(ltoa(result))
  }
}
main()
