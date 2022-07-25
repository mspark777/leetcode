function search (nums: number[], target: number, first: boolean): number {
  let result = -1
  let left = 0
  let right = nums.length - 1
  while (left <= right) {
    const mid = Math.trunc((left + right) / 2)
    const num = nums[mid]
    if (num > target) {
      right = mid - 1
    } else if (num < target) {
      left = mid + 1
    } else {
      result = mid
      if (first) {
        right = mid - 1
      } else {
        left = mid + 1
      }
    }
  }

  return result
}

export function searchRange (nums: number[], target: number): number[] {
  const first = search(nums, target, true)
  const last = search(nums, target, false)
  return [first, last]
}
