// import { createRequire } from 'module'
import { findPaths } from './solution.mjs'

async function main () {
  const inputs = [
    {
      m: 2,
      n: 2,
      maxMove: 2,
      startRow: 0,
      startColumn: 0
    },
    {
      m: 1,
      n: 3,
      maxMove: 3,
      startRow: 0,
      startColumn: 1
    },
    {
      m: 3,
      n: 2,
      maxMove: 5,
      startRow: 1,
      startColumn: 1
    }
  ]

  for (const input of inputs) {
    const result = findPaths(input.m, input.n, input.maxMove, input.startRow, input.startColumn)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
