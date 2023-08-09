/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} nums
  * @param {number} threshold
  * @returns {number}
  */
function countValidPairs (nums, threshold) {
  let index = 0
  let count = 0
  const last = nums.length - 1
  while (index < last) {
    const n0 = nums[index]
    const n1 = nums[index + 1]
    const diff = n1 - n0
    if (diff <= threshold) {
      count += 1
      index += 1
    }

    index += 1
  }

  return count
}

/**
  * @param {number[]} nums
  * @param {number} p
  * @returns {number}
  */
function minimizeMax (nums, p) {
  nums.sort((a, b) => a - b)

  let left = 0
  let right = nums.at(-1) - nums.at(0)
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)

    if (countValidPairs(nums, mid) >= p) {
      right = mid
    } else {
      left = mid + 1
    }
  }

  return left
}

function main () {
  const inputs = [
    { nums: [10, 1, 2, 7, 1, 3], p: 2 },
    { nums: [4, 2, 1, 2], p: 1 }
  ]

  for (const { nums, p } of inputs) {
    const result = minimizeMax(nums, p)
    console.log(result)
  }
}
main()
