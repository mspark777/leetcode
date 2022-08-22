/**
 * @param {number[]} nums
 * @param {number} k
 * @return {boolean}
 */
function containsNearbyDuplicate (nums, k) {
  const indexMap = new Map()
  for (let i = 0; i < nums.length; i += 1) {
    const key = nums[i]
    const idx = indexMap.get(key)
    if ((idx != null) && ((i - idx) <= k)) {
      return true
    }

    indexMap.set(key, i)
  }

  return false
}

async function main () {
  const inputs = [
    {
      nums: [1, 2, 3, 1],
      k: 3
    },
    {
      nums: [1, 0, 1, 1],
      k: 1
    },
    {
      nums: [1, 2, 3, 1, 2, 3],
      k: 2
    }
  ]

  for (const { nums, k } of inputs) {
    const result = containsNearbyDuplicate(nums, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
