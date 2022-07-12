function dfs (nums, sums, index, target) {
  if (index >= nums.length) {
    return sums.slice(1).every(s => s === target)
  }

  for (let i = 0; i < 4; i += 1) {
    if ((sums[i] + nums[index]) > target) {
      continue
    }

    sums[i] += nums[index]
    if (dfs(nums, sums, index + 1, target)) {
      return true
    }
    sums[i] -= nums[index]
  }

  return false
}

export function makesquare (matchsticks) {
  const sum = matchsticks.reduce((acc, cur) => acc + cur, 0)
  if ((sum % 4) !== 0) {
    return false
  }

  matchsticks.sort((a, b) => b - a)
  return dfs(matchsticks, [0, 0, 0, 0], 0, Math.trunc(sum / 4))
}
