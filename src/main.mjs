// import { createRequire } from 'module'
import { partition, ListNode } from './solution.mjs'

function arrToList (arr) {
  const head = new ListNode()
  let tail = head

  for (const n of arr) {
    tail.next = new ListNode(n)
    tail = tail.next
  }

  return head.next
}

function listToArr (node) {
  const arr = []
  while (node) {
    arr.push(node.val)
    node = node.next
  }

  return arr
}

async function main () {
  const inputs = [
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
