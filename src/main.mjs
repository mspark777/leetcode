// import { createRequire } from 'module'
import { hasCycle, ListNode } from './solution.mjs'

function arrtolist (arr, pos) {
  const head = new ListNode()
  let tail = head
  let cycle = null
  for (let i = 0; i < arr.length; i += 1) {
    const node = new ListNode(arr[i])
    tail.next = node
    tail = node
    if (i === pos) {
      cycle = node
    }
  }

  tail.next = cycle
  return head.next
}

async function main () {
  const inputs = [
    {
      head: [3, 2, 0, -4],
      pos: 1
    },
    {
      head: [1, 2],
      pos: 0
    },
    {
      head: [1],
      pos: -1
    }
  ]

  for (const input of inputs) {
    const result = hasCycle(arrtolist(input.head, input.pos))
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
