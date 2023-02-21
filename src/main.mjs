/**
  * @param {number[]} nums
  * @returns {number}
  */
function singleNonDuplicate (nums) {
  let left = 0n
  let right = BigInt(nums.length - 1)
  while (left < right) {
    const middle = (left + right) / 2n
    const pos = Number(middle)
    if ((middle & 1n) === 1n) {
      if (nums[pos] !== nums[pos + 1]) {
        left = middle + 1n
      } else {
        right = middle
      }
    } else {
      if (nums[pos] === nums[pos + 1]) {
        left = middle + 1n
      } else {
        right = middle
      }
    }
  }

  return nums[Number(left)]
}

async function main () {
  const inputs = [
    [1, 1, 2, 3, 3, 4, 4, 8, 8],
    [3, 3, 7, 7, 10, 11, 11]
  ]

  for (const nums of inputs) {
    const result = singleNonDuplicate(nums)
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
