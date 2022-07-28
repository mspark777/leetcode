export class ListNode {
  val: number
  // eslint-disable-next-line no-use-before-define
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function getIntersectionNode (headA: ListNode | null, headB: ListNode | null): ListNode | null {
  let a = headA
  let b = headB
  while (a !== b) {
    a = a ? a.next : headB
    b = b ? b.next : headA
  }

  return a
}
