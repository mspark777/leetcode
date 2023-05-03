function filter (nums1: number[], nums2: number[]): number[] {
  const set1 = new Set<number>(nums2)
  const set2 = new Set(nums1.filter(n => !set1.has(n)))
  return [...set2]
}

function findDifference (nums1: number[], nums2: number[]): number[][] {
  return [filter(nums1, nums2), filter(nums2, nums1)]
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
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
