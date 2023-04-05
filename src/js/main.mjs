/**
  * @param {number[]} nums
  * @returns {number}
  */
function minimizeArrayValue (nums) {
  let result = 0n
  let prefixSum = 0n

  for (let i = 0; i < nums.length; i += 1) {
    prefixSum += BigInt(nums[i])
    const j = BigInt(i)
    const sum = (prefixSum + j) / (j + 1n)
    if (sum > result) {
      result = sum
    }
  }

  return Number(result)
}

async function main () {
  const inputs = [
    [3, 7, 1, 6],
    [10, 1]
  ]

  for (const nums of inputs) {
    const result = minimizeArrayValue(nums)
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
