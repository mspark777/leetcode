/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @returns {number}
 */
function findLength (nums1, nums2) {
  let result = 0
  const lengths = new Array(nums1.length + 1).fill([])
    .map(() => new Array(nums2.length + 1).fill(0))

  for (let i = nums1.length - 1; i >= 0; i -= 1) {
    for (let j = nums2.length - 1; j >= 0; j -= 1) {
      if (nums1[i] !== nums2[j]) {
        continue
      }

      const length = lengths[i + 1][j + 1] + 1
      lengths[i][j] = length
      result = Math.max(result, length)
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      nums1: [1, 2, 3, 2, 1],
      nums2: [3, 2, 1, 4, 7]
    },
    {
      nums1: [0, 0, 0, 0, 0],
      nums2: [0, 0, 0, 0, 0]
    }
  ]

  for (const { nums1, nums2 } of inputs) {
    const result = findLength(nums1, nums2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
