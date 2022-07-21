export class ListNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function reverseBetween (head: ListNode | null, left: number, right: number): ListNode | null {
  if (!head) {
    return head
  }

  let cur: ListNode | null = head
  let prev: ListNode | null = null
  while (left > 1) {
    prev = cur
    cur = cur!.next
    left -= 1
    right -= 1
  }

  const tail = cur
  const con = prev
  let third: ListNode | null = null
  while (right > 0) {
    third = cur!.next
    cur!.next = prev
    prev = cur
    cur = third
    right -= 1
  }

  if (con) {
    con.next = prev
  } else {
    head = prev
  }
  tail!.next = cur
  return head
}
