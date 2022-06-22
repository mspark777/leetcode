import { findKthLargest } from './solution.mjs'

async function main () {
  const inputs = [
    { nums: [3, 2, 1, 5, 6, 4], k: 2 },
    { nums: [3, 2, 3, 1, 2, 4, 5, 5, 6], k: 4 }
  ]

  for (const input of inputs) {
    const result = findKthLargest(input.nums, input.k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
