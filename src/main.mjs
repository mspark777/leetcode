// import { createRequire } from 'module'
import { getIntersectionNode, ListNode } from './solution.mjs'

function createInput (numsa, numsb, skipa, skipb) {
  const headA = arrtolist(numsa.slice(0, skipa))
  const headB = arrtolist(numsb.slice(0, skipb))
  if (!headA || !headB) {
    return { headA, headB }
  }

  const remain = arrtolist(numsa.slice(skipa))

  let tail = headA
  while (tail && tail.next) {
    tail = tail.next
  }
  tail.next = remain

  tail = headB
  while (tail?.next) {
    tail = tail.next
  }
  tail.next = remain

  return { headA, headB }
}

function arrtolist (nums) {
  const head = new ListNode()
  let tail = head
  for (const n of nums) {
    tail.next = new ListNode(n)
    tail = tail.next
  }

  return head.next
}

function listtoarr (node) {
  const nums = []
  while (node) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

async function main () {
  const inputs = [
    {
      ...createInput([4, 1, 8, 4, 5], [5, 6, 1, 8, 4, 5], 2, 3)
    },
    {
      ...createInput([1, 9, 1, 2, 4], [3, 2, 4], 3, 1)
    },
    {
      ...createInput([2, 6, 4], [1, 5], 3, 2)
    }
  ]

  for (const input of inputs) {
    const result = getIntersectionNode(input.headA, input.headB)
    console.log(listtoarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
