function maxIceCream(costs: number[], coins: number): number {
  costs.sort((a, b) => a - b)

  let result = 0

  for (const cost of costs) {
    if (coins >= cost) {
      coins -= cost
      result += 1
    } else {
      break
    }
  }

  return result
}

interface Input {
  readonly costs: number[]
  readonly coins: number
}

async function main(): Promise<void> {
  const inputs: Input[] = [
    {costs: [1, 3, 2, 4, 1], coins: 7},
    {costs: [10, 6, 8, 7, 7, 8], coins: 5},
    {costs: [1, 6, 3, 1, 2, 5], coins: 20}
  ]

  for (const {costs, coins} of inputs) {
    const result = maxIceCream(costs, coins)
    console.log(result)
  }
}

await main()
