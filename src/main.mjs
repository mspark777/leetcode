
class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function arrtolist (nums) {
  const dummy = new ListNode()
  let tail = dummy
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return dummy.next
}

function listtoarr (node) {
  const nums = []
  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

/**
 * @param {ListNode | null} head
 * @param {number} n
 * @returns {ListNode | null}
*/
function removeNthFromEnd (head, n) {
  if (head == null) {
    return null
  }

  let right = head
  for (let i = 0; i < n; i += 1) {
    right = right?.next ?? null
  }

  if (right == null) {
    return head.next
  }

  let left = head
  while (right.next != null) {
    right = right.next
    left = left?.next ?? null
  }

  if (left != null) {
    left.next = left?.next?.next ?? null
  }

  return head
}

async function main () {
  const inputs = [{
    head: arrtolist([1, 2, 3, 4, 5]),
    n: 2
  }, {
    head: arrtolist([1]),
    n: 1
  }, {
    head: arrtolist([1, 2]),
    n: 1
  }]

  for (const { head, n } of inputs) {
    const result = removeNthFromEnd(head, n)
    console.log(listtoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
