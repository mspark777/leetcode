// import { createRequire } from 'module'
import { maximumUnits } from './solution.mjs'

async function main () {
  const inputs = [
    { boxTypes: [[1, 3], [2, 2], [3, 1]], truckSize: 4 },
    { boxTypes: [[5, 10], [2, 5], [4, 7], [3, 9]], truckSize: 10 }
  ]

  for (const input of inputs) {
    const result = maximumUnits(input.boxTypes, input.truckSize)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
