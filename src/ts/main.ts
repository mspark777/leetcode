function mostPoints (questions: number[][]): number {
  const qlen = questions.length
  const last = qlen - 1
  const dp = new Array<number>(qlen).fill(0)
  dp[last] = questions[last][0]

  for (let i = last - 1; i >= 0; i -= 1) {
    const [point, power] = questions[i]
    dp[i] = point

    if ((i + power) < last) {
      dp[i] += dp[i + power + 1]
    }
    dp[i] = Math.max(dp[i], dp[i + 1])
  }

  return dp[0]
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[3, 2], [4, 3], [4, 4], [2, 5]],
    [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]
  ]

  for (const questions of inputs) {
    const result = mostPoints(questions)
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
