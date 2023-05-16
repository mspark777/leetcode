import { ListNode, newList, unused, listToArr } from './utils.mjs'

unused(ListNode)

/**
  * @param {ListNode | null} head
  * @returns {ListNode | null}
  */
function swapPairs (head) {
  if (head?.next == null) {
    return head
  }

  const dummy = new ListNode()
  let prev = dummy
  let curr = head

  while (curr?.next != null) {
    prev.next = curr.next
    curr.next = prev.next.next
    prev.next.next = curr

    prev = curr
    curr = curr.next
  }

  return dummy.next
}

async function main () {
  const inputs = [
    newList([1, 2, 3, 4]),
    newList([]),
    newList([1])
  ]

  for (const head of inputs) {
    const result = swapPairs(head)
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
