/**
 * @param {number[]} nums
 * @returns {number}
*/
function rob (nums) {
  let robbed = 0
  let noRobbed = 0
  for (const n of nums) {
    const temp = noRobbed
    noRobbed = Math.max(noRobbed, robbed)
    robbed = n + temp
  }

  return Math.max(noRobbed, robbed)
}

async function main () {
  const inputs = [
    [1, 2, 3, 1],
    [2, 7, 9, 3, 1]
  ]

  for (const nums of inputs) {
    const result = rob(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
