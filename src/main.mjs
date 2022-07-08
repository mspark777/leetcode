// import { createRequire } from 'module'
import { minCost } from './solution.mjs'

async function main () {
  const inputs = [
    {
      houses: [0, 0, 0, 0, 0],
      cost: [[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]],
      m: 5,
      n: 2,
      target: 3
    },
    {
      houses: [0, 2, 1, 2, 0],
      cost: [[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]],
      m: 5,
      n: 2,
      target: 3
    },
    {
      houses: [3, 1, 2, 3],
      cost: [[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]],
      m: 4,
      n: 3,
      target: 3
    }
  ]

  for (const input of inputs) {
    const result = minCost(input.houses, input.cost, input.m, input.n, input.target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
