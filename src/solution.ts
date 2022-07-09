export function maxResult (nums: number[], k: number): number {
  const dp = new Array(nums.length)
  dp[0] = nums[0]
  const dequeue: number[] = [0]
  for (let i = 1; i < nums.length; i += 1) {
    if (dequeue[0] < (i - k)) {
      dequeue.shift()
    }
    dp[i] = nums[i] + dp[dequeue[0]]
    while (dequeue.length > 0 && dp[dequeue.at(-1)!] <= dp[i]) {
      dequeue.pop()
    }

    dequeue.push(i)
  }

  return dp.at(-1) as number
}
