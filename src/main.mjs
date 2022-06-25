// import { createRequire } from 'module'
import { checkPossibility } from './solution.mjs'

async function main () {
  const inputs = [
    { nums: [4, 2, 3] },
    { nums: [4, 2, 1] }
  ]

  for (const input of inputs) {
    const result = checkPossibility(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
