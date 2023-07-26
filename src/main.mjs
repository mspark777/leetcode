/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} dist
  * @param {number} speed
  * @returns {number}
  */
function timeRequired (dist, speed) {
  let time = 0
  for (const [i, d] of dist.entries()) {
    const t = d / speed
    time += i === dist.length - 1 ? t : Math.ceil(t)
  }

  return time
}

/**
  * @param {number[]} dist
  * @param {number} hour
  * @returns {number}
  */
function minSpeedOnTime (dist, hour) {
  let left = 1
  let right = 10000001
  let minSpeed = -1

  while (left <= right) {
    const mid = Math.trunc((left + right) / 2)
    if (timeRequired(dist, mid) <= hour) {
      minSpeed = mid
      right = mid - 1
    } else {
      left = mid + 1
    }
  }

  return minSpeed
}

function main () {
  const inputs = [
    { dist: [1, 3, 2], hour: 6 },
    { dist: [1, 3, 2], hour: 2.7 },
    { dist: [1, 3, 2], hour: 1.9 },
    { dist: [1, 1, 100000], hour: 2.01 },
    { dist: [90, 94, 72, 85, 92, 63, 20, 25, 38, 28, 8, 75, 95, 70, 8, 96, 41, 8, 7, 75, 62, 65, 68, 21, 8, 66, 11, 24, 9, 77, 9, 87, 31, 52, 16, 73, 32, 75, 77, 6, 80, 11, 54, 85, 75, 73, 67, 41, 34, 27, 86, 92, 41, 31, 34, 51, 17, 86, 83, 39, 59, 41, 97, 10, 2, 59, 80, 73, 13, 10, 69, 28, 65, 34, 17, 45, 9], hour: 393.18 }
  ]

  for (const { dist, hour } of inputs) {
    const result = minSpeedOnTime(dist, hour)
    console.log(result)
  }
}
main()
