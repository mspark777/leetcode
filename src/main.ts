import { maximumUnits } from './solution'

interface Input {
  readonly boxTypes: number[][]
  readonly truckSize: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
