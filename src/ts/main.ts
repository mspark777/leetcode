
function dp (days: number[], costs: number[], memos: number[], durations: number[], i: number): number {
  if (i >= days.length) {
    return 0
  } else if (memos[i] !== 0) {
    return memos[i]
  }

  let result = Number.MAX_SAFE_INTEGER
  let j = i
  for (const [d, duration] of durations.entries()) {
    while (j < days.length) {
      const k = days[i] + duration
      if (days[j] < k) {
        j += 1
      } else {
        break
      }
    }

    const recv = dp(days, costs, memos, durations, j)
    result = Math.min(result, recv + costs[d])
  }

  memos[i] = result
  return result
}

function mincostTickets (days: number[], costs: number[]): number {
  const memos = new Array<number>(days.length).fill(0)
  const durations = [1, 7, 30]

  return dp(days, costs, memos, durations, 0)
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 4, 6, 7, 8, 20], [2, 7, 15]],
    [[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15]]
  ]

  for (const [days, costs] of inputs) {
    const result = mincostTickets(days, costs)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
