/**
  * @param {number[]} nums
  * @param {number[]} cost
  * @param {number} base
  * @returns {number}
  */
function getCost (nums, cost, base) {
  let result = 0
  for (const [i, num] of nums.entries()) {
    result += Math.abs(num - base) * cost[i]
  }

  return result
}

/**
  * @param {number[]} nums
  * @param {number[]} cost
  * @returns {number}
  */
function minCost (nums, cost) {
  let left = 1000001
  let right = 0
  for (const num of nums) {
    left = Math.min(left, num)
    right = Math.max(right, num)
  }

  let result = getCost(nums, cost, nums[0])
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const cost1 = getCost(nums, cost, mid)
    const cost2 = getCost(nums, cost, mid + 1)
    result = Math.min(cost1, cost2)

    if (cost1 > cost2) {
      left = mid + 1
    } else {
      right = mid
    }
  }

  return result
}

function main () {
  const inputs = [
    { nums: [1, 3, 5, 2], cost: [2, 3, 1, 14] },
    { nums: [2, 2, 2, 2, 2], cost: [4, 2, 8, 1, 3] }
  ]

  for (const { nums, cost } of inputs) {
    const result = minCost(nums, cost)
    console.log(result)
  }
}
main()
