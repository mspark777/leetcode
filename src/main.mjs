/**
 * @param {number[]} nums
 * @returns {boolean}
 */
function canJump (nums) {
  let last = nums.length - 1
  for (let i = nums.length - 2; i >= 0; i -= 1) {
    const cur = i + nums[i]
    if (cur >= last) {
      last = i
    }
  }

  return last < 1
}

async function main () {
  const inputs = [
    [2, 3, 1, 1, 4],
    [3, 2, 1, 0, 4]
  ]

  for (const nums of inputs) {
    const result = canJump(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
