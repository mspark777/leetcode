/**
  * @param {number[]} arr
  * @param {number} difference
  * @returns {number}
  */
function longestSubsequence (arr, difference) {
  /** @type {Map<number, number>} */
  const dp = new Map()
  let result = 1

  for (const num of arr) {
    const before = dp.get(num - difference) ?? 0
    const now = before + 1
    dp.set(num, now)

    result = Math.max(result, now)
  }

  return result
}

function main () {
  const inputs = [
    { arr: [1, 2, 3, 4], difference: 1 },
    { arr: [1, 3, 5, 7], difference: 1 },
    { arr: [1, 5, 7, 8, 5, 3, 4, 2, 1], difference: -2 }
  ]

  for (const { arr, difference } of inputs) {
    const result = longestSubsequence(arr, difference)
    console.log(result)
  }
}
main()
