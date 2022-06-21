/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
export const maxOperations = function (nums, k) {
  nums.sort((a, b) => a - b)

  let performed = 0
  let head = 0
  let tail = nums.length - 1
  while (head < tail) {
    const nh = nums[head]
    const nt = nums[tail]
    const sum = nh + nt
    if (sum > k) {
      tail -= 1
    } else if (sum < k) {
      head += 1
    } else {
      performed += 1
      head += 1
      tail -= 1
    }
  }

  return performed
}
