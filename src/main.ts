function intersection (nums1: number[], nums2: number[]): number[] {
  const filter1 = new Set(nums1)
  const filter2 = new Set(nums2)
  const result: number[] = []

  for (const num of filter1) {
    if (filter2.has(num)) {
      result.push(num)
    }
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 2, 2, 1], [2, 2]],
    [[4, 9, 5], [9, 4, 9, 8, 4]]
  ]

  for (const [nums1, nums2] of inputs) {
    const result = intersection(nums1, nums2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
