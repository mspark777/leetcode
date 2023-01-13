function intersect (nums1: number[], nums2: number[]): number[] {
  const counts = new Map<number, number>()
  for (const num of nums1) {
    const count = counts.get(num) ?? 0
    counts.set(num, count + 1)
  }

  const result: number[] = []
  for (const num of nums2) {
    const count = counts.get(num) ?? 0
    if (count > 0) {
      counts.set(num, count - 1)
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
    const result = intersect(nums1, nums2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
