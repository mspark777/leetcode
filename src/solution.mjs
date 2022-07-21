export class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

export function reverseBetween (head, left, right) {
  if (!head) {
    return head
  }

  let cur = head
  let prev = null
  while (left > 1) {
    prev = cur
    cur = cur.next
    left -= 1
    right -= 1
  }

  const tail = cur
  const con = prev
  let third = null
  while (right > 0) {
    third = cur.next
    cur.next = prev
    prev = cur
    cur = third
    right -= 1
  }

  if (con) {
    con.next = prev
  } else {
    head = prev
  }
  tail.next = cur
  return head
}
