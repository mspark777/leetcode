/**
  * @param {number[]} nums
  * @param {number} target
  * @returns {number}
  */
function searchInsert (nums, target) {
  if (target < nums.at(0)) {
    return 0
  } else if (target > nums.at(-1)) {
    return nums.length
  }

  let begin = 0n
  let end = BigInt(nums.length)
  while (begin < end) {
    const middle = (begin + end) / 2n
    const pos = Number(middle)
    const num = nums[pos]
    if (num < target) {
      begin = middle + 1n
    } else if (num > target) {
      end = middle
    } else {
      return pos
    }
  }

  return Number(begin)
}

async function main () {
  const inputs = [
    { nums: [1, 3, 5, 6], target: 5 },
    { nums: [1, 3, 5, 6], target: 2 },
    { nums: [1, 3, 5, 6], target: 7 }
  ]

  for (const { nums, target } of inputs) {
    const result = searchInsert(nums, target)
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
