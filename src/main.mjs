/**
  * @param {number[]} nums
  * @returns {number}
  */
function jump (nums) {
  let result = 0
  let curend = 0
  let curfar = 0

  for (let i = 0; i < nums.length - 1; i += 1) {
    curfar = Math.max(curfar, i + nums[i])

    if (i === curend) {
      result += 1
      curend = curfar
    }
  }

  return result
}

async function main () {
  const inputs = [
    [2, 3, 1, 1, 4],
    [2, 3, 0, 1, 4]
  ]

  for (const nums of inputs) {
    const result = jump(nums)
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
