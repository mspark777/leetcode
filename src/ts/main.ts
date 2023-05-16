import { ListNode, listToArr, newList } from './utils'

function swapPairs (head: ListNode | null): ListNode | null {
  if (head?.next == null) {
    return head
  }

  const dummy = new ListNode()
  let prev: ListNode | null = dummy
  let curr: ListNode | null = head

  while (curr?.next != null) {
    prev.next = curr.next
    curr.next = prev.next.next
    prev.next.next = curr

    prev = curr
    curr = curr.next
  }

  return dummy.next
}

async function main (): Promise<void> {
  const inputs: Array<ListNode | null> = [
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
