/**
 * @param {number[]} nums
 * @param {number} index
 * @param {number[]} sequence
 * @param {Map<string, number[]>} result
 * @returns {undefined}
 */
function backtrack (nums, index, sequence, result) {
  if (index === nums.length) {
    if (sequence.length >= 2) {
      const key = sequence.join()
      result.set(key, sequence.slice())
    }
  } else {
    const num = nums[index]
    const lastseq = sequence.at(-1) ?? num

    if (lastseq <= num) {
      sequence.push(num)
      backtrack(nums, index + 1, sequence, result)
      sequence.pop()
    }
    backtrack(nums, index + 1, sequence, result)
  }
}

/**
 * @param {number[]} nums
 * @returns {number[][]}
 */
function findSubsequences (nums) {
  const result = new Map()
  const sequence = []
  backtrack(nums, 0, sequence, result)
  return [...result.values()]
}

async function main () {
  const inputs = [
    [4, 6, 7, 7],
    [4, 4, 3, 2, 1]
  ]

  for (const nums of inputs) {
    const result = findSubsequences(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
