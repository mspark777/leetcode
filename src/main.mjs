/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number}
 */
function search(nums, target) {
  let left = 0
  let right = nums.length - 1
  while (left <= right) {
    const mid = Math.round((left + right) / 2)
    if (nums[mid] === target) {
      return mid
    }

    if (nums[mid] >= nums[left]) {
      if ((target >= nums[left]) && (target < nums[mid])) {
        right = mid - 1
      } else {
        left = mid + 1
      }
    } else {
      if ((target > nums[mid]) && (target <= nums[right])) {
        left = mid + 1
      } else {
        right = mid - 1
      }
    }
  }

  return -1
}

async function main() {
  const inputs = [
    {nums: [4, 5, 6, 7, 0, 1, 2], target: 0},
    {nums: [4, 5, 6, 7, 0, 1, 2], target: 3},
    {nums: [1], target: 0}
  ]

  for (const {nums, target} of inputs) {
    const result = search(nums, target)
    console.log(result)
  }
}

await main()
