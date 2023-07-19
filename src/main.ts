import '@total-typescript/ts-reset'

function eraseOverlapIntervals (intervals: number[][]): number {
  intervals.sort((a, b) => {
    const ay = a[1] as number
    const by = b[1] as number
    return ay - by
  })

  let result = 0
  let k = Number.MIN_SAFE_INTEGER
  for (const interval of intervals) {
    const x = interval[0] as number
    const y = interval[1] as number

    if (x >= k) {
      k = y
    } else {
      result += 1
    }
  }

  return result
}

function main (): void {
  const inputs = [
    [[1, 2], [2, 3], [3, 4], [1, 3]],
    [[1, 2], [1, 2], [1, 2]],
    [[1, 2], [2, 3]]
  ]

  for (const intervals of inputs) {
    const result = eraseOverlapIntervals(intervals)
    console.log(result)
  }
}
main()
