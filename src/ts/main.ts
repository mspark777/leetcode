import { type ListNode, newList } from './utils'

function pairSum (head: ListNode | null): number {
  let nodeCount = 0
  for (let node = head; node != null; node = node.next) {
    nodeCount += 1
  }

  let i = 0
  const nums = new Array<number>(nodeCount)
  for (let node = head; node != null; node = node.next) {
    nums[i] = node.val
    i += 1
  }

  i = 0
  let j = nodeCount - 1
  let result = 0
  while (i < j) {
    result = Math.max(result, nums[i] + nums[j])
    i += 1
    j -= 1
  }

  return result
}

async function main (): Promise<void> {
  const inputs: Array<ListNode | null> = [
    newList([5, 4, 2, 1]),
    newList([4, 2, 2, 3]),
    newList([1, 100000])
  ]

  for (const head of inputs) {
    const result = pairSum(head)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
