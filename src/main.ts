import { partition, ListNode } from './solution'

interface Input {
  readonly head: number[]
  readonly x: number
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
      head: [1, 4, 3, 2, 5, 2],
      x: 3
    },
    {
      head: [2, 1],
      x: 2
    },
    {
      head: [],
      x: 1
    }
  ]

  for (const input of inputs) {
    const result = partition(arrToList(input.head), input.x)
    console.log(listToArr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
