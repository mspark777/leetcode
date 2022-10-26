/**
 * @param {number[]} nums
 * @param {number} k
 * @returns {boolean}
*/
function checkSubarraySum (nums, k) {
  const map = new Map([[0, 0]])
  let sum = 0
  for (let i = 0; i < nums.length; i += 1) {
    sum += nums[i]
    const mod = sum % k
    const memo = map.get(mod)
    if (memo == null) {
      map.set(mod, i + 1)
    } else if (memo < i) {
      return true
    }
  }

  return false
}

async function main () {
  const inputs = [
    {
      nums: [23, 2, 4, 6, 7],
      k: 6
    },
    {
      nums: [23, 2, 6, 4, 7],
      k: 6
    },
    {

      nums: [23, 2, 6, 4, 7],
      k: 13
    }
  ]

  for (const { nums, k } of inputs) {
    const result = checkSubarraySum(nums, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
