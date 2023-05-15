import { ListNode, newList, unused, listToArr } from './utils.mjs'

unused(ListNode)

/**
  * @param {ListNode | null} head
  * @param {number} k
  * @returns {ListNode | null}
  */
function swapNodes (head, k) {
  let left = head
  for (let i = 1; i < k; i += 1) {
    left = left?.next ?? null
  }

  if (left == null) {
    return head
  }

  let right = head
  for (let i = left; i?.next != null; i = i.next) {
    right = right?.next ?? null
  }

  if (right == null) {
    return head
  }

  const l = left.val
  const r = right.val
  left.val = r
  right.val = l

  return head
}

async function main () {
  const inputs = [
    { head: newList([1, 2, 3, 4, 5]), k: 2 },
    { head: newList([7, 9, 6, 6, 7, 8, 3, 0, 9, 5]), k: 5 }
  ]

  for (const { head, k } of inputs) {
    const result = swapNodes(head, k)
    console.log(listToArr(result))
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
