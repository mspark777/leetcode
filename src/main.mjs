/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} start
  * @param {number[]} combinations
  * @param {number[][]} results
  * @param {number} n
  * @param {number} k
  * @returns {undefined}
  */
function backtrack (start, combinations, results, n, k) {
  if (combinations.length === k) {
    results.push([...combinations])
    return
  }

  for (let i = start; i <= n; i += 1) {
    combinations.push(i)
    backtrack(i + 1, combinations, results, n, k)
    combinations.pop()
  }
}

/**
  * @param {number} n
  * @param {number} k
  * @returns {number[][]}
  */
function combine (n, k) {
  /** @type {number[][]} */
  const results = []
  /** @type {number[]} */
  const combinations = []
  backtrack(1, combinations, results, n, k)

  return results
}

function main () {
  const inputs = [
    [4, 2],
    [1, 1]
  ]

  for (const [n, k] of inputs) {
    const result = combine(n, k)
    console.log(result)
  }
}
main()
