import { listToArr, newList, type ListNode } from './utils'

function swapNodes (head: ListNode | null, k: number): ListNode | null {
  let left = head
  for (let i = 1; i < k; i += 1) {
    left = left?.next ?? null
  }

  if (left == null) {
    return head
  }

  let right = head
  for (let i: ListNode | null = left; i?.next != null; i = i.next) {
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

interface Input {
  readonly head: ListNode | null
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
