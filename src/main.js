/**
 * @param {number[]} nums
 * @returns {number}
 */
function maxSubarraySumCircular (nums) {
  let curMax = 0
  let curMin = 0
  let sum = 0
  let maxSum = nums[0]
  let minSum = nums[0]

  for (const num of nums) {
    curMax = Math.max(curMax, 0) + num
    maxSum = Math.max(maxSum, curMax)
    curMin = Math.min(curMin, 0) + num
    minSum = Math.min(minSum, curMin)
    sum += num
  }

  return sum === minSum ? maxSum : Math.max(maxSum, sum - minSum)
}

async function main () {
  const inputs = [
    [1, -2, 3, -2],
    [5, -3, 5],
    [-3, -2, -3]
  ]

  for (const nums of inputs) {
    const result = maxSubarraySumCircular(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
