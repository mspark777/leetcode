// import { createRequire } from 'module'
import { numSubmatrixSumTarget } from './solution.mjs'

async function main () {
  const inputs = [
    {
      matrix: [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
      target: 0
    },
    {
      matrix: [[1, -1], [-1, 1]],
      target: 0
    },
    {
      matrix: [[904]],
      target: 0
    }
  ]

  for (const input of inputs) {
    const result = numSubmatrixSumTarget(input.matrix, input.target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
