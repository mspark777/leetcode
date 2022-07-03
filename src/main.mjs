// import { createRequire } from 'module'
import { wiggleMaxLength } from './solution.mjs'

async function main () {
  const inputs = [
    {
      nums: [1, 7, 4, 9, 2, 5]
    },
    {
      nums: [1, 17, 5, 10, 13, 15, 10, 5, 16, 8]
    },
    {
      nums: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
  ]

  for (const input of inputs) {
    const result = wiggleMaxLength(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
