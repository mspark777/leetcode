function profitableSchemes (n: number, minProfit: number, group: number[], profits: number[]): number {
  const MOD = 1000000007
  const MAXN = 101
  const dp: number[][][] = new Array(MAXN)
  for (let i = 0; i < MAXN; i += 1) {
    dp[i] = new Array(MAXN)
    for (let j = 0; j < MAXN; j += 1) {
      dp[i][j] = new Array(MAXN).fill(0)
    }
  }

  for (let count = 0; count <= n; count++) {
    dp[group.length][count][minProfit] = 1
  }

  for (let index = group.length - 1; index >= 0; index -= 1) {
    for (let count = 0; count <= n; count += 1) {
      for (let profit = 0; profit <= minProfit; profit += 1) {
        dp[index][count][profit] = dp[index + 1][count][profit]
        if (count + group[index] <= n) {
          dp[index][count][profit] =
            (dp[index][count][profit] + dp[index + 1][count + group[index]][Math.min(minProfit, profit + profits[index])]) % MOD
        }
      }
    }
  }

  return dp[0][0][0]
}

interface Input {
  readonly n: number
  readonly minProfit: number
  readonly group: number[]
  readonly profit: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: 5, minProfit: 3, group: [2, 2], profit: [2, 3] },
    { n: 10, minProfit: 5, group: [2, 3, 5], profit: [6, 7, 8] }
  ]

  for (const { n, minProfit, group, profit } of inputs) {
    const result = profitableSchemes(n, minProfit, group, profit)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
