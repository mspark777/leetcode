/**
  * @param {number[]} nums
  * @returns {number}
  */
function longestSubarray (nums) {
  let zeroCount = 0
  let result = 0
  let start = 0

  for (const [i, num] of nums.entries()) {
    zeroCount += num === 0 ? 1 : 0

    while (zeroCount > 1) {
      zeroCount -= nums[start] === 0 ? 1 : 0
      start += 1
    }

    result = Math.max(result, i - start)
  }

  return result
}

function main () {
  const inputs = [
    [1, 1, 0, 1],
    [0, 1, 1, 1, 0, 1, 1, 0, 1],
    [1, 1, 1]
  ]

  for (const nums of inputs) {
    const result = longestSubarray(nums)
    console.log(result)
  }
}
main()
