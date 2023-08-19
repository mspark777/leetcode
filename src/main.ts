import '@total-typescript/ts-reset'

class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function mod (left: number, right: number): number {
  return Number(BigInt(left) % BigInt(right))
}

function rotateRight (head: ListNode | null, k: number): ListNode | null {
  if (head?.next == null) {
    return head
  } else if (k < 1) {
    return head
  }

  let cur = head
  let len = 1
  while (cur.next != null) {
    len += 1
    cur = cur.next
  }

  cur.next = head
  k = mod(k, len)
  for (let i = len - k; i > 0; i -= 1) {
    cur = cur?.next as ListNode
  }

  head = cur.next
  cur.next = null

  return head
}

function main (): void {
  const inputs: Array<[number[], number]> = [
    [[1, 2, 3, 4, 5], 2],
    [[0, 1, 2], 4]
  ]

  for (const [head, k] of inputs) {
    const result = rotateRight(null, 0)
    console.log(result)
  }
}
main()
