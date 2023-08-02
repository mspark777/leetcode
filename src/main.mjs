/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {Set<number>} permutations
  * @param {number[]} nums
  * @param {number[][]} results
  * @returns {undefined}
  */
function backtrack (permutations, nums, results) {
  if (permutations.size === nums.length) {
    results.push([...permutations])
    return
  }

  for (const num of nums) {
    if (permutations.has(num)) {
      continue
    }

    permutations.add(num)
    backtrack(permutations, nums, results)
    permutations.delete(num)
  }
}

/**
  * @param {number[]} nums
  * @returns {number[][]}
  */
function permute (nums) {
  /** @type {number[][]} */
  const results = []
  /** @type {Set<number>} */
  const permutations = new Set()
  backtrack(permutations, nums, results)

  return results
}

function main () {
  const inputs = [
    [1, 2, 3],
    [0, 1],
    [1]
  ]

  for (const nums of inputs) {
    const result = permute(nums)
    console.log(result)
  }
}
main()
