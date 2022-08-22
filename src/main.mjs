class ListNode {
  constructor (val, next) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function reverseRecursively (node) {
  if (node == null) {
    return node
  }

  const next = node.next
  if (next == null) {
    return node
  }

  const newNode = reverseRecursively(next)
  next.next = node
  node.next = null
  return newNode
}

function reverseIteratively (node) {
  let next = null
  let prev = null
  while (node != null) {
    next = node.next
    node.next = prev
    prev = node
    node = next
  }

  return prev
}

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
function reverseList (head) {
  return reverseRecursively(head)
}

function arrToList (nums) {
  const dummy = new ListNode()
  let tail = dummy
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return dummy.next
}

function listToArr (node) {
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
      nums: [1, 2, 3, 4, 5]
    },
    {
      nums: [1, 2]
    },
    {
      nums: []
    }
  ]

  for (const { nums } of inputs) {
    const result = reverseList(arrToList(nums))
    console.log(listToArr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
