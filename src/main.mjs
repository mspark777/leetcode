// import { createRequire } from 'module'
import { minMoves2 } from './solution.mjs'

async function main () {
  const inputs = [
    { nums: [1, 2, 3] },
    { nums: [1, 10, 2, 9] }
  ]

  for (const input of inputs) {
    const result = minMoves2(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
