/**
  * @param {number} target
  * @param {number[]} nums
  * @returns {number}
  */
function minSubArrayLen (target, nums) {
  let left = 0
  let sum = 0
  let result = Number.MAX_SAFE_INTEGER

  for (let right = 0; right < nums.length; right += 1) {
    sum += nums[right]
    while (sum >= target) {
      result = Math.min(result, right - left + 1)
      sum -= nums[left]
      left += 1
    }
  }

  return result < Number.MAX_SAFE_INTEGER ? result : 0
}

function main () {
  const inputs = [
    { target: 7, nums: [2, 3, 1, 2, 4, 3] },
    { target: 4, nums: [1, 4, 4] },
    { target: 11, nums: [1, 1, 1, 1, 1, 1, 1, 1] }
  ]

  for (const { target, nums } of inputs) {
    const result = minSubArrayLen(target, nums)
    console.log(result)
  }
}
main()
