function swap (nums, left, right) {
  const t = nums[left]
  nums[left] = nums[right]
  nums[right] = t
}

function partition (nums, left, right) {
  const pivot = nums[right]
  let j = left
  for (let i = left; i < right; i += 1) {
    if (nums[i] < pivot) {
      swap(nums, i, j)
      j += 1
    }
  }

  swap(nums, j, right)
  return j
}

export function findKthLargest (nums, k) {
  const target = nums.length - k
  let left = 0
  let right = nums.length - 1
  while (true) {
    const mid = partition(nums, left, right)
    if (mid > target) {
      right = mid - 1
    } else if (mid < target) {
      left = mid + 1
    } else {
      return nums[target]
    }
  }
}
