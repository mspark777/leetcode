/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function combinationSum4 (nums, target) {
  const result = new Array(target + 1).fill(0)
  result[0] = 1
  for (let i = 1; i <= target; i += 1) {
    for (const num of nums) {
      if (i >= num) {
        result[i] += result[i - num]
      }
    }
  }

  return result[target]
}

async function main () {
  const inputs = [
    {
      nums: [1, 2, 3],
      target: 4
    },
    {
      nums: [9],
      target: 3
    }
  ]

  for (const { nums, target } of inputs) {
    const result = combinationSum4(nums, target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
