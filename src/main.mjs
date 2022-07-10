// import { createRequire } from 'module'
import { minCostClimbingStairs } from './solution.mjs'

async function main () {
  const inputs = [
    {
      cost: [10, 15, 20]
    },
    {
      cost: [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
    }
  ]

  for (const input of inputs) {
    const result = minCostClimbingStairs(input.cost)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
