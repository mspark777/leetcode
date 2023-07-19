/**
  * @param {number[][]} intervals
  * @returns {number}
  */
function eraseOverlapIntervals (intervals) {
  intervals.sort((a, b) => a[1] - b[1])

  let result = 0
  let k = Number.MIN_SAFE_INTEGER
  for (const [x, y] of intervals) {
    if (x >= k) {
      k = y
    } else {
      result += 1
    }
  }

  return result
}

function main () {
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
