// import { createRequire } from 'module'
import { maxResult } from './solution.mjs'

async function main () {
  const inputs = [
    {
      nums: [1, -1, -2, 4, -7, 3],
      k: 2
    },
    {
      nums: [10, -5, -2, 4, 0, 3],
      k: 3
    },
    {
      nums: [1, -5, -20, 4, -1, 3, -6, -3],
      k: 2
    }
  ]

  for (const input of inputs) {
    const result = maxResult(input.nums, input.k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
