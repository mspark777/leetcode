class ListNode {
  val: number
  next: ListNode | null
  constructor (val?: number, next?: ListNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.next = (next === undefined ? null : next)
  }
}

function arrlist (nums: number[]): ListNode | null {
  const temp = new ListNode()
  let tail = temp

  for (const num of nums) {
    tail.next = new ListNode(num)
    tail = tail.next
  }

  return temp.next
}

function listarr (node: ListNode | null): number[] {
  const nums: number[] = []

  while (node != null) {
    nums.push(node.val)
    node = node.next
  }

  return nums
}

function deleteMiddle (head: ListNode | null): ListNode | null {
  if (head?.next == null) {
    return null
  }

  let slow = head
  let fast = head.next.next

  while (fast?.next != null) {
    slow = slow.next as ListNode
    fast = fast.next.next
  }

  slow.next = slow.next?.next ?? null
  return head
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [1, 3, 4, 7, 1, 2, 6],
    [1, 2, 3, 4],
    [2, 1]
  ]

  for (const nums of inputs) {
    const result = deleteMiddle(arrlist(nums))
    console.log(listarr(result))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
