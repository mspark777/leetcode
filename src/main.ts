import { reverseBetween, ListNode } from './solution'

interface Input {
  readonly head: number[]
  readonly left: number
  readonly right: number
}

function arrToList (arr: number[]): ListNode | null {
  const head = new ListNode()
  let tail = head

  for (const n of arr) {
    tail.next = new ListNode(n)
    tail = tail.next
  }

  return head.next
}

function listToArr (node: ListNode | null): number[] {
  const arr: number[] = []
  while (node) {
    arr.push(node.val)
    node = node.next
  }

  return arr
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      head: [1, 2, 3, 4, 5],
      left: 2,
      right: 4
    },
    {
      head: [5],
      left: 1,
      right: 1
    },
    {
      head: [],
      left: 1,
      right: 100
    }
  ]

  for (const input of inputs) {
    const result = reverseBetween(
      arrToList(input.head),
      input.left,
      input.right
    )
    console.log(listToArr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
