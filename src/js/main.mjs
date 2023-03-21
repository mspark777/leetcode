/**
  * @param {number[]} nums
  * @returns {number}
  */
function zeroFilledSubarray (nums) {
  let result = 0
  let sub = 0

  for (const num of nums) {
    if (num === 0) {
      sub += 1
    } else {
      sub = 0
    }

    result += sub
  }

  return result
}

async function main () {
  const inputs = [
    [1, 3, 0, 0, 2, 0, 0, 4],
    [0, 0, 0, 2, 0, 0],
    [2, 10, 2019]
  ]

  for (const nums of inputs) {
    const result = zeroFilledSubarray(nums)
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
