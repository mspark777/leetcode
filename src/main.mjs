/**
  * @param {number} i
  * @param {number[]} candidates
  * @param {number[]} temp
  * @param {number} target
  * @param {number[][]} result
  * @returns {undefined}
  */
function solve (i, candidates, temp, target, result) {
  if (target === 0) {
    result.push(temp.slice())
    return
  }

  if (target < 0) {
    return
  }

  if (i >= candidates.length) {
    return
  }

  solve(i + 1, candidates, temp, target, result)

  const candidate = candidates[i]
  temp.push(candidate)
  solve(i, candidates, temp, target - candidate, result)
  temp.pop()
}

/**
  * @param {number[]} candidates
  * @param {number} target
  * @returns {number[][]}
  */
function combinationSum (candidates, target) {
  const result = []

  solve(0, candidates, [], target, result)
  return result
}

function main () {
  const inputs = [
    { candidates: [2, 3, 6, 7], target: 7 },
    { candidates: [2, 3, 5], target: 8 },
    { candidates: [2], target: 1 }
  ]

  for (const { candidates, target } of inputs) {
    const result = combinationSum(candidates, target)
    console.log(result)
  }
}
main()
