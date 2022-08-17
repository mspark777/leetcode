class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

/**
 * @param {ListNode} head
 * @param {number} val
 * @return {ListNode}
 */
function removeElements (head, val) {
  const dummy = new ListNode(-1, head)
  let cur = dummy

  while (cur != null) {
    while ((cur.next != null) && (cur.next.val === val)) {
      cur.next = cur.next.next
    }

    cur = cur.next
  }

  return dummy.next
}

function arrtolist (nums) {
  const head = new ListNode()
  let tail = head
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return head.next
}

function listtoarr (node) {
  const nums = []
  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

async function main () {
  const inputs = [
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
