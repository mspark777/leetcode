class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function reverseRecursively (node: ListNode | null): ListNode | null {
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

function reverseIteratively (node: ListNode | null): ListNode | null {
  let next: ListNode | null = null
  let prev: ListNode | null = null
  while (node != null) {
    next = node.next
    node.next = prev
    prev = node
    node = next
  }

  return prev
}

function reverseList (head: ListNode | null): ListNode | null {
  return reverseRecursively(head)
}

interface Input {
  readonly nums: number[]
}

function arrToList (nums: number[]): ListNode | null {
  const dummy = new ListNode()
  let tail = dummy
  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return dummy.next
}

function listToArr (node: ListNode | null): number[] {
  const nums: number[] = []

  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
