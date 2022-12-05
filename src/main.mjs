class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

/**
 * @param {ListNode | null} head
 * @returns {ListNode | null}
*/
function middleNode (head) {
  let slow = head
  let fast = head
  while (fast?.next != null) {
    slow = slow?.next ?? null
    fast = fast.next.next
  }

  return slow
}

function arrtolist (nums) {
  const head = new ListNode()
  let tail = head
  for (const val of nums) {
    tail.next = new ListNode(val)
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
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 5, 6]
  ]

  for (const nums of inputs) {
    const result = middleNode(arrtolist(nums))
    console.log(listtoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
