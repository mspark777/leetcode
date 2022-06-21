/**
 * @param {number[]} nums
 * @return {number}
 */
export const findUnsortedSubarray = function (nums) {
  let head = 0
  let tail = nums.length - 1
  while (head < tail) {
    if (nums[head] <= nums[head + 1]) {
      head += 1
    } else {
      break
    }
  }

  if (head === tail) {
    return 0
  }

  while (head < tail) {
    if (nums[tail - 1] <= nums[tail]) {
      tail -= 1
    } else {
      break
    }
  }

  if (head === tail) {
    return 0
  }

  let max = nums[head]
  let min = nums[tail]
  for (let i = head + 1; i < tail; i += 1) {
    max = Math.max(max, nums[i])
    min = Math.min(min, nums[i])
  }

  for (let i = 0; i < head; i += 1) {
    if (min < nums[i]) {
      head = i
    }
  }

  for (let i = nums.length - 1; i > tail; i -= 1) {
    if (nums[i] < max) {
      tail = i
    }
  }

  return tail - head + 1
}
