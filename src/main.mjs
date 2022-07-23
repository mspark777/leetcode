// import { createRequire } from 'module'
import { countSmaller } from './solution.mjs'

async function main () {
  const inputs = [
    {
      nums: [5, 2, 6, 1]
    },
    {
      nums: [-1]
    },
    {
      nums: [-1, -1]
    }
  ]

  for (const input of inputs) {
    const result = countSmaller(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
