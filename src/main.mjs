/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[][]} intervals
  * @returns {number[][]}
  */
function merge (intervals) {
  intervals.sort((a, b) => a[0] - b[0])

  /** @type {number[][]} */
  const result = [intervals[0]]
  for (const interval of intervals.slice(1)) {
    const [start, end] = interval
    const last = result.at(-1)
    const lastEnd = last[1]
    if (lastEnd < start) {
      result.push(interval)
    } else {
      last[1] = Math.max(lastEnd, end)
    }
  }

  return result
}

function main () {
  const inputs = [
    [[1, 3], [2, 6], [8, 10], [15, 18]],
    [[1, 4], [4, 5]]
  ]

  for (const intervals of inputs) {
    const result = merge(intervals)
    console.log(result)
  }
}
main()
