import { newCycleList, type ListNode } from './lib'

function detectCycle (head: ListNode | null): ListNode | null {
  let fast = head
  let slow = head
  while (fast?.next != null) {
    fast = fast.next.next
    slow = (slow as ListNode).next

    if (fast === slow) {
      break
    }
  }

  if (fast?.next == null) {
    return null
  }

  fast = head

  while (fast !== slow) {
    fast = fast?.next ?? null
    slow = slow?.next ?? null
  }

  return fast
}

async function main (): Promise<void> {
  const inputs: ListNode[] = [
    newCycleList([3, 2, 0, -4], 1),
    newCycleList([1, 2], 0),
    newCycleList([1], -1)
  ]

  for (const head of inputs) {
    const result = detectCycle(head)
    console.log(result?.val)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
