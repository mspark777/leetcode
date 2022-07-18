// import { createRequire } from 'module'
import { maxProfit } from './solution.mjs'

async function main () {
  const inputs = [
    {
      prices: [7, 1, 5, 3, 6, 4]
    },
    {
      prices: [7, 6, 4, 3, 1]
    }
  ]

  for (const input of inputs) {
    const result = maxProfit(input.prices)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
