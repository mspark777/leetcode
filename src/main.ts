import '@total-typescript/ts-reset'

function merge (intervals: number[][]): number[][] {
  intervals.sort((as, bs) => {
    const a = as[0] as number
    const b = bs[0] as number
    return a - b
  })

  const result: number[][] = [intervals[0] as number[]]
  for (const interval of intervals.slice(1)) {
    const [start, end] = interval as [number, number]
    const last = result.at(-1) as number[]
    const lastEnd = last[1] as number
    if (lastEnd < start) {
      result.push(interval)
    } else {
      last[1] = Math.max(lastEnd, end)
    }
  }

  return result
}

function main (): void {
  const inputs: number[][][] = [
    [[1, 3], [2, 6], [8, 10], [15, 18]],
    [[1, 4], [4, 5]]
  ]

  for (const intervals of inputs) {
    const result = merge(intervals)
    console.log(result)
  }
}
main()
