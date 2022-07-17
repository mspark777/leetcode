// import { createRequire } from 'module'
import { kInversePairs } from './solution.mjs'

async function main () {
  const inputs = [
    {
      n: 3,
      k: 0
    },
    {
      n: 3,
      k: 1
    }
  ]

  for (const input of inputs) {
    const result = kInversePairs(input.n, input.k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
