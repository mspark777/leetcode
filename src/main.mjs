/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} nums
  * @returns {boolean}
  */
function PredictTheWinner (nums) {
  const n = nums.length
  const dp = nums.slice()

  for (let diff = 1; diff < n; diff += 1) {
    for (let left = 0; left < n - diff; left += 1) {
      const right = left + diff
      const lnum = nums[left]
      const rnum = nums[right]
      const ldp = dp[left]
      const rdp = dp[left + 1]
      dp[left] = Math.max(lnum - rdp, rnum - ldp)
    }
  }

  const first = dp[0]
  return first >= 0
}

function main () {
  const inputs = [
    [1, 5, 2],
    [1, 5, 233, 7]
  ]

  for (const nums of inputs) {
    const result = PredictTheWinner(nums)
    console.log(result)
  }
}
main()
