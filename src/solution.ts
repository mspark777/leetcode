export function checkPossibility (nums: number[]): boolean {
  if (nums.length < 3) {
    return true
  }

  let count = 0
  for (let i = 1; i < nums.length; i += 1) {
    const prev = nums[i - 1]
    const cur = nums[i]
    if (cur < prev) {
      count += 1
      if (count > 1) {
        return false
      }

      if (i > 1) {
        if ((nums[i - 2] <= cur)) {
          nums[i - 1] = nums[i - 2]
        } else {
          nums[i] = prev
        }
      }
    }
  }

  return count < 2
}
