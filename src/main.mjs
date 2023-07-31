/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} candidates
  * @param {number[]} combinations
  * @param {number} remain
  * @param {number} cur
  * @param {number[][]} results
  * @returns {undefined}
  */
function backtrack (candidates, combinations, remain, cur, results) {
  if (remain === 0) {
    results.push(combinations.slice())
    return
  }

  for (let next = cur; next < candidates.length; next += 1) {
    if ((next > cur) && (candidates[next] === candidates[next - 1])) {
      continue
    }

    /** @type {number}  */
    const pick = candidates[next]
    const nextRemain = remain - pick
    if (nextRemain < 0) {
      break
    }

    combinations.push(pick)
    backtrack(candidates, combinations, nextRemain, next + 1, results)
    combinations.pop()
  }
}

/**
  * @param {number[]} candidates
  * @param {number} target
  * @returns {number[][]}
  */
function combinationSum2 (candidates, target) {
  /** @type {number[][]} */
  const results = []
  /** @type {number[]} */
  const combinations = []

  candidates.sort((a, b) => a - b)
  backtrack(candidates, combinations, target, 0, results)
  return results
}

function main () {
  const inputs = [
    { candidates: [10, 1, 2, 7, 6, 1, 5], target: 8 },
    { candidates: [2, 5, 2, 1, 2], target: 5 }
  ]

  for (const { candidates, target } of inputs) {
    const result = combinationSum2(candidates, target)
    console.log(result)
  }
}
main()
