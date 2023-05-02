/**
  * @param {number[]} nums
  * @returns {number}
  */
function arraySign (nums) {
  const result = nums.reduce((acc, cur) => acc * cur)
  if (result < 0) {
    return -1
  } else if (result > 0) {
    return 1
  }

  return 0
}

async function main () {
  const inputs = [
    [-1, -2, -3, -4, 3, 2, 1],
    [1, 5, 0, 2, -3],
    [-1, 1, -1, 1, -1]
  ]

  for (const nums of inputs) {
    const result = arraySign(nums)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
