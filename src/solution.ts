export function wiggleMaxLength (nums: number[]): number {
  if (nums.length < 2) {
    return nums.length
  }

  let down = 1
  let up = 1
  for (let i = 1; i < nums.length; i += 1) {
    const cur = nums[i]
    const prev = nums[i - 1]
    if (cur > prev) {
      up = down + 1
    } else if (cur < prev) {
      down = up + 1
    }
  }

  return Math.max(up, down)
}
