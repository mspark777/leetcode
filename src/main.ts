import '@total-typescript/ts-reset'

function change (amount: number, coins: number[]): number {
  const dp = new Array<number>(amount + 1).fill(0)
  dp[0] = 1

  for (let i = coins.length - 1; i >= 0; i -= 1) {
    const coin = coins[i] as number
    for (let j = coin; j <= amount; j += 1) {
      dp[j] += dp[j - coin] as number
    }
  }

  return dp[amount] as number
}

interface Input {
  readonly amount: number
  readonly coins: number[]
}

function main (): void {
  const inputs: Input[] = [
    { amount: 5, coins: [1, 2, 5] },
    { amount: 3, coins: [2] },
    { amount: 10, coins: [10] }
  ]

  for (const { amount, coins } of inputs) {
    const result = change(amount, coins)
    console.log(result)
  }
}
main()
