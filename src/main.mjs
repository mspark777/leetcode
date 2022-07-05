// import { createRequire } from 'module'
import { longestConsecutive } from './solution.mjs'

async function main () {
  const inputs = [
    {
      nums: [100, 4, 200, 1, 3, 2]
    },
    {
      nums: [0, 3, 7, 2, 5, 8, 4, 6, 0, 1]
    }
  ]

  for (const input of inputs) {
    const result = longestConsecutive(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
