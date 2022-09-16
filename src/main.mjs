/**
 * @param {number[]} nums
 * @param {number[]} multipliers
 * @returns {number}
 */
function maximumScore (nums, multipliers) {
  const n = nums.length
  const m = multipliers.length
  const dp = new Array(m + 1).fill(0)

  for (let op = m - 1; op >= 0; op -= 1) {
    const nextRow = dp.slice()

    for (let left = op; left >= 0; left -= 1) {
      const n0 = multipliers[op] * nums[left] + nextRow[left + 1]
      const n1 = multipliers[op] * nums[n - 1 - (op - left)] + nextRow[left]
      dp[left] = Math.max(n0, n1)
    }
  }

  return dp[0]
}

async function main () {
  const inputs = [
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
