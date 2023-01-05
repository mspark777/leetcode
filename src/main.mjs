/**
 * @param {number[][]} points
 * @returns {number}
 */
function findMinArrowShots(points) {
  points.sort(([_l0, r0], [_l1, r1]) => r0 - r1)

  let result = 1
  let prev = 0

  for (let cur = 1; cur < points.length; cur += 1) {
    if (points[cur][0] > points[prev][1]) {
      result += 1
      prev = cur
    }
  }

  return result
}

async function main() {
  const inputs = [
    [[10, 16], [2, 8], [1, 6], [7, 12]],
    [[1, 2], [3, 4], [5, 6], [7, 8]],
    [[1, 2], [2, 3], [3, 4], [4, 5]]
  ]

  for (const points of inputs) {
    const result = findMinArrowShots(points)
    console.log(result)
  }
}

await main()
