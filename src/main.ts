function maximumScore (nums: number[], multipliers: number[]): number {
  const n = nums.length
  const m = multipliers.length
  const dp = new Array<number>(m + 1).fill(0)

  for (let op = m - 1; op >= 0; op -= 1) {
    const row = dp.slice()

    for (let left = op; left >= 0; left -= 1) {
      const n0 = multipliers[op] * nums[left] + row[left + 1]
      const n1 = multipliers[op] * nums[n - 1 - (op - left)] + row[left]
      dp[left] = Math.max(n0, n1)
    }
  }

  return dp[0]
}

interface Input {
  readonly nums: number[]
  readonly multipliers: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3],
      multipliers: [3, 2, 1]
    },
    {
      nums: [-5, -3, -3, -2, 7, 1],
      multipliers: [-10, -5, 3, 4, 6]
    }
  ]

  for (const { nums, multipliers } of inputs) {
    const result = maximumScore(nums, multipliers)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
