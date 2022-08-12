function swap (nums, i, j) {
  const temp = nums[i]
  nums[i] = nums[j]
  nums[j] = temp
}

function reverse (nums, start) {
  let i = start
  let j = nums.length - 1
  while (i < j) {
    swap(nums, i, j)
    i += 1
    j -= 1
  }
}

/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
function nextPermutation (nums) {
  let i = nums.length - 2
  while ((i >= 0) && nums[i] >= nums[i + 1]) {
    i -= 1
  }

  if (i >= 0) {
    let j = nums.length - 1
    while (nums[i] >= nums[j]) {
      j -= 1
    }

    swap(nums, i, j)
  }

  reverse(nums, i + 1)
}

async function main () {
  const inputs = [
    {
      nums: [1, 2, 3]
    },
    {
      nums: [3, 2, 1]
    },
    {
      nums: [1, 1, 5]
    }
  ]

  for (const { nums } of inputs) {
    nextPermutation(nums)
    console.log(nums)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
