export function minOperations (nums: number[], x: number): number {
  let target = -x
  for (const n of nums) {
    target += n
  }

  if (target < 0) {
    return -1
  } else if (target === 0) {
    return nums.length
  }

  let sum = 0
  let result = -1
  for (let left = 0, right = 0; right < nums.length; right += 1) {
    sum += nums[right]
    while (sum > target) {
      sum -= nums[left]
      left += 1
    }

    if (sum === target) {
      result = Math.max(result, right - left + 1)
    }
  }

  return result === -1 ? -1 : nums.length - result
}
