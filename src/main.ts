function mergeIntervals (intervals: number[][], newInterval: number[]): void {
  for (let i = 0; i < intervals.length; i += 1) {
    if (intervals[i][0] < newInterval[0]) {
      continue
    }

    intervals.splice(i, 0, newInterval)
    return
  }

  intervals.push(newInterval)
}

function insert (intervals: number[][], newInterval: number[]): number[][] {
  mergeIntervals(intervals, newInterval)

  const result: number[][] = [intervals[0]]

  for (let i = 1; i < intervals.length; i += 1) {
    const last = result.at(-1) as number[]
    const interval = intervals[i]
    if (interval[0] > last[1]) {
      result.push(interval)
    } else {
      last[1] = Math.max(last[1], interval[1])
    }
  }

  return result
}

interface Input {
  readonly intervals: number[][]
  readonly newInterval: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { intervals: [[1, 3], [6, 9]], newInterval: [2, 5] },
    { intervals: [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], newInterval: [4, 8] }
  ]

  for (const { intervals, newInterval } of inputs) {
    const result = insert(intervals, newInterval)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
