/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @param {number} k
 * @returns {number}
 */
function findKthLargest (nums, k) {
  nums.sort((a, b) => b - a)
  return nums[k - 1]
}

function main () {
  const inputs = [
    { nums: [3, 2, 1, 5, 6, 4], k: 2 },
    { nums: [3, 2, 3, 1, 2, 4, 5, 5, 6], k: 4 }
  ]

  for (const { nums, k } of inputs) {
    const result = findKthLargest(nums, k)
    console.log(result)
  }
}
main()
