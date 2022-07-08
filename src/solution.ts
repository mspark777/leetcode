function createMemo (row: number, col: number, val: number): number[][] {
  const memo = new Array<number[]>(row)
  for (let i = 0; i < row; i += 1) {
    memo[i] = new Array<number>(col).fill(val)
  }

  return memo
}

export function minCost (houses: number[], cost: number[][], m: number, n: number, target: number): number {
  const MAX_COST = 1000001
  let prevMemo = createMemo(target + 1, n, MAX_COST)

  for (let color = 1; color <= n; color += 1) {
    if (houses[0] === color) {
      prevMemo[1][color - 1] = 0
    } else if (!houses[0]) {
      prevMemo[1][color - 1] = cost[0][color - 1]
    }
  }

  for (let house = 1; house < m; house += 1) {
    const memo = createMemo(target + 1, n, MAX_COST)

    for (let neighborhoods = 1; neighborhoods <= Math.min(target, house + 1); neighborhoods += 1) {
      for (let color = 1; color <= n; color += 1) {
        if (houses[house] && color !== houses[house]) {
          continue
        }

        let currCost = MAX_COST
        for (let prevColor = 1; prevColor <= n; prevColor += 1) {
          if (prevColor !== color) {
            currCost = Math.min(currCost, prevMemo[neighborhoods - 1][prevColor - 1])
          } else {
            currCost = Math.min(currCost, prevMemo[neighborhoods][color - 1])
          }
        }

        const costToPaint = houses[house] ? 0 : cost[house][color - 1]
        memo[neighborhoods][color - 1] = currCost + costToPaint
      }
    }
    prevMemo = memo
  }

  let minCost = MAX_COST
  for (let color = 1; color <= n; color += 1) {
    minCost = Math.min(minCost, prevMemo[target][color - 1])
  }

  return minCost === MAX_COST ? -1 : minCost
}
