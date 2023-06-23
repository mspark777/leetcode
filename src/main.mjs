/**
  * @param {number[]} nums
  * @returns {number}
  */
function longestArithSeqLength (nums) {
  let result = 0
  const dp = new Array(nums.length)
  for (let right = 0; right < nums.length; right += 1) {
    dp[right] = new Map()
    for (let left = 0; left < right; left += 1) {
      const diff = nums[left] - nums[right]
      const rmap = dp[right]
      const lmap = dp[left]

      const curLen = (lmap.get(diff) ?? 1) + 1
      rmap.set(diff, curLen)
      result = Math.max(result, curLen)
    }
  }

  return result
}

function main () {
  const inputs = [
    [3, 6, 9, 12],
    [9, 4, 7, 2, 10],
    [20, 1, 15, 3, 10, 5, 8]
  ]

  for (const nums of inputs) {
    const result = longestArithSeqLength(nums)
    console.log(result)
  }
}
main()
