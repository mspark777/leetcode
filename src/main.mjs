/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} nums
  * @param {number} target
  * @returns {boolean}
  */
function search (nums, target) {
  if (nums.length < 1) {
    return false
  }

  let left = 0
  let right = nums.length
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const mnum = nums[mid]
    if (mnum === target) {
      return true
    }

    const lnum = nums[left]
    if (lnum === mnum) {
      left += 1
      continue
    }

    const pivotArray = lnum <= mnum
    const targetArray = lnum <= target
    if (pivotArray !== targetArray) {
      if (pivotArray) {
        left = mid + 1
      } else {
        right = mid
      }
    } else {
      if (mnum < target) {
        left = mid + 1
      } else {
        right = mid
      }
    }
  }

  return false
}

function main () {
  const inputs = [
    { nums: [2, 5, 6, 0, 0, 1, 2], target: 0 },
    { nums: [2, 5, 6, 0, 0, 1, 2], target: 3 },
    { nums: [1, 0, 1, 1, 1], target: 0 }
  ]

  for (const { nums, target } of inputs) {
    const result = search(nums, target)
    console.log(result)
  }
}
main()
