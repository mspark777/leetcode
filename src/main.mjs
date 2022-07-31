// import { createRequire } from 'module'
import { NumArray } from './solution.mjs'

async function main () {
  const inputs = [
    {
      nums: [1, 3, 5]
    }
  ]

  for (const input of inputs) {
    const narr = new NumArray(input.nums)
    console.log(narr.sumRange(0, 2))
    narr.update(1, 2)
    console.log(narr.sumRange(0, 2))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
