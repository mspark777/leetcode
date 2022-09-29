/**
 * @param {number[]} arr
 * @param {number} k
 * @param {number} x
 * @returns {number[]}
 */
function findClosestElements (arr, k, x) {
  let left = 0
  let right = arr.length - k

  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const a = arr[mid + k] - x
    const b = x - arr[mid]
    if (a < b) {
      left = mid + 1
    } else {
      right = mid
    }
  }

  return arr.slice(left, left + k)
}

async function main () {
  const inputs = [
    {
      arr: [1, 2, 3, 4, 5],
      k: 4,
      x: 3
    }, {
      arr: [1, 2, 3, 4, 5],
      k: 4,
      x: -1
    }
  ]

  for (const { arr, k, x } of inputs) {
    const result = findClosestElements(arr, k, x)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
