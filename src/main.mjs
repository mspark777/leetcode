/**
 * @param {number[]} nums
 * @return {number}
 */
function majorityElement (nums) {
  let count = 0
  let candidate = nums[0]

  for (const num of nums) {
    if (count < 1) {
      candidate = num
    }

    count += (num === candidate) ? 1 : -1
  }

  return candidate
}

async function main () {
  const inputs = [
    {
      nums: [3, 2, 3]
    },
    {
      nums: [2, 2, 1, 1, 1, 2, 2]
    }
  ]

  for (const { nums } of inputs) {
    const result = majorityElement(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
