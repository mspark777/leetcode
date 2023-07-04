/**
  * @param {number[]} nums
  * @return {number}
  */
function firstMissingPositive (nums) {
  for (let i = 0; i < nums.length; i += 1) {
    while ((nums[i] > 0) && (nums[i] <= nums.length) && (nums[nums[i] - 1] !== nums[i])) {
      const temp = nums[i]
      nums[i] = nums[temp - 1]
      nums[temp - 1] = temp
    }
  }

  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] !== (i + 1)) {
      return i + 1
    }
  }

  return nums.length + 1
}

function main () {
  const inputs = [
    [1, 2, 0],
    [3, 4, -1, 1],
    [7, 8, 9, 11, 12]
  ]

  for (const nums of inputs) {
    const result = firstMissingPositive(nums)
    console.log(result)
  }
}
main()
