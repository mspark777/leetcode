/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @returns {undefined}
 */
function mergeIntervals (intervals, newInterval) {
  for (let i = 0; i < intervals.length; i += 1) {
    if (intervals[i][0] < newInterval[0]) {
      continue
    }

    intervals.splice(i, 0, newInterval)
    return
  }

  intervals.push(newInterval)
}

/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @returns {number[][]}
 */
function insert (intervals, newInterval) {
  mergeIntervals(intervals, newInterval)

  const result = [intervals[0]]

  for (let i = 1; i < intervals.length; i += 1) {
    const last = result.at(-1)
    const interval = intervals[i]
    if (interval[0] > last[1]) {
      result.push(interval)
    } else {
      last[1] = Math.max(last[1], interval[1])
    }
  }

  return result
}

async function main () {
  const inputs = [
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
