export function minCostClimbingStairs (cost) {
  let step0 = cost[0]
  let step1 = cost[1]
  for (let i = 2; i < cost.length; i += 1) {
    const cur = cost[i] + Math.min(step0, step1)
    step0 = step1
    step1 = cur
  }

  return Math.min(step0, step1)
}
