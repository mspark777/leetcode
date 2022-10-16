function minDifficulty (jobDifficulty: number[], d: number): number {
  const days = jobDifficulty.length
  if (days < d) {
    return -1
  }

  const dp = new Array<number>(days + 1).fill(0)

  for (let i = days - 1; i >= 0; i -= 1) {
    dp[i] = Math.max(dp[i + 1], jobDifficulty[i])
  }

  for (let i = 2; i <= d; i += 1) {
    const remain = days - i
    for (let j = 0; j <= remain; j += 1) {
      let maxd = 0
      dp[j] = Number.MAX_SAFE_INTEGER
      for (let k = j; k <= remain; k += 1) {
        maxd = Math.max(maxd, jobDifficulty[k])
        dp[j] = Math.min(dp[j], maxd + dp[k + 1])
      }
    }
  }

  return dp[0]
}

interface Input {
  readonly jobDifficulty: number[]
  readonly d: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { jobDifficulty: [6, 5, 4, 3, 2, 1], d: 2 },
    { jobDifficulty: [9, 9, 9], d: 4 },
    { jobDifficulty: [1, 1, 1], d: 3 }
  ]

  for (const { jobDifficulty, d } of inputs) {
    const result = minDifficulty(jobDifficulty, d)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
