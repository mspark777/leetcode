class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function arrlist (nums) {
  const temp = new ListNode()
  let tail = temp

  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return temp.next
}

function listarr (node) {
  const nums = []

  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

/**
 * @param {ListNode | null} head
 * @returns {ListNode | null}
*/
function deleteMiddle (head) {
  if (head?.next == null) {
    return null
  }

  let slow = head
  let fast = head.next.next

  while (fast?.next != null) {
    slow = slow.next
    fast = fast.next.next
  }

  slow.next = slow.next?.next ?? null
  return head
}

async function main () {
  const inputs = [
    [1, 3, 4, 7, 1, 2, 6],
    [1, 2, 3, 4],
    [2, 1]
  ]

  for (const nums of inputs) {
    const result = deleteMiddle(arrlist(nums))
    console.log(listarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
