export function maximumUniqueSubarray (nums: number[]): number {
  let result = Number.MIN_SAFE_INTEGER
  let sum = 0
  const visits = new Map<number, number>()
  for (let left = 0, right = 0; right < nums.length; right += 1) {
    const rn = nums[right]
    if (visits.has(rn)) {
      const visited = visits.get(rn) as number
      while (left <= visited) {
        sum -= nums[left]
        left += 1
      }
    }

    sum += rn
    result = Math.max(result, sum)
    visits.set(rn, right)
  }

  return result
}
