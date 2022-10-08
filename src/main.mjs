/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number}
*/
function threeSumClosest (nums, target) {
  nums.sort((a, b) => a - b)

  let result = Number.MIN_SAFE_INTEGER
  let diffresult = Number.MAX_SAFE_INTEGER
  for (let i = 0; i < nums.length; i += 1) {
    const ni = nums[i]

    let j = i + 1
    let k = nums.length - 1
    while (j < k) {
      const nj = nums[j]
      const nk = nums[k]
      const sum = ni + nj + nk
      const diffsum = Math.abs(target - sum)

      if (diffsum < diffresult) {
        result = sum
        diffresult = diffsum
      }

      if (sum < target) {
        j += 1
      } else if (sum > target) {
        k -= 1
      } else {
        return sum
      }
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      nums: [-1, 2, 1, -4],
      target: 1
    },
    {
      nums: [0, 0, 0],
      target: 1
    }
  ]

  for (const { nums, target } of inputs) {
    const result = threeSumClosest(nums, target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
