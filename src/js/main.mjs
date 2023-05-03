/**
  * @param {number[]} nums1
  * @param {number[]} nums2
  * @returns {number[]}
  */
function filter (nums1, nums2) {
  const set1 = new Set(nums2)
  const set2 = new Set(nums1.filter(n => !set1.has(n)))
  return [...set2]
}

/**
  * @param {number[]} nums1
  * @param {number[]} nums2
  * @returns {number[][]}
  */
function findDifference (nums1, nums2) {
  return [filter(nums1, nums2), filter(nums2, nums1)]
}

async function main () {
  const inputs = [
    [[1, 2, 3], [2, 4, 6]],
    [[1, 2, 3, 3], [1, 1, 2, 2]]
  ]

  for (const [nums1, nums2] of inputs) {
    const result = findDifference(nums1, nums2)
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
