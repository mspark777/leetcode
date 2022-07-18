import { maxProfit } from './solution'

interface Input {
  readonly prices: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      prices: [7, 1, 5, 3, 6, 4]
    },
    {
      prices: [7, 6, 4, 3, 1]
    },
    {
      prices: [3, 3, 5, 0, 0, 3, 1, 4]
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
