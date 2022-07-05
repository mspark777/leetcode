export function longestConsecutive (nums) {
  const numSet = new Set(nums)
  let longest = 0

  for (const num of numSet) {
    if (numSet.has(num - 1)) {
      continue
    }

    let cur = num + 1
    let count = 1
    while (numSet.has(cur)) {
      count += 1
      cur += 1
    }

    longest = Math.max(longest, count)
  }

  return longest
}
