export function minMoves2 (nums) {
  nums.sort((a, b) => a - b)
  const midIndex = Math.trunc(nums.length / 2)
  const mid = nums[midIndex]
  return nums.reduce((acc, cur) => acc + Math.abs(mid - cur), 0)
}
