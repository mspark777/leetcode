import '@total-typescript/ts-reset'

function PredictTheWinner (nums: number[]): boolean {
  const n = nums.length
  const dp = nums.slice()

  for (let diff = 1; diff < n; diff += 1) {
    for (let left = 0; left < n - diff; left += 1) {
      const right = left + diff
      const lnum = nums[left] as number
      const rnum = nums[right] as number
      const ldp = dp[left] as number
      const rdp = dp[left + 1] as number
      dp[left] = Math.max(lnum - rdp, rnum - ldp)
    }
  }

  const first = dp[0] as number
  return first >= 0
}

function main (): void {
  const inputs: number[][] = [
    [1, 5, 2],
    [1, 5, 233, 7]
  ]

  for (const nums of inputs) {
    const result = PredictTheWinner(nums)
    console.log(result)
  }
}
main()
