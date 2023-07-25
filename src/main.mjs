/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} arr
  * @returns {number}
  */
function peakIndexInMountainArray (arr) {
  let left = 0
  let right = arr.length - 1
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const l = arr.at(mid)
    const r = arr.at(mid + 1)
    if (l < r) {
      left = mid + 1
    } else {
      right = mid
    }
  }

  return left
}

function main () {
  const inputs = [
    [0, 1, 0],
    [0, 2, 1, 0],
    [0, 10, 5, 2]
  ]

  for (const arr of inputs) {
    const result = peakIndexInMountainArray(arr)
    console.log(result)
  }
}
main()
