function maxUncrossedLines (nums1: number[], nums2: number[]): number {
  const len1 = nums1.length
  const len2 = nums2.length

  const dp = new Array<number>(len2 + 1).fill(0)
  let dpPrev = new Array<number>(len2 + 1).fill(0)

  for (let i = 1; i <= len1; i += 1) {
    for (let j = 1; j <= len2; j += 1) {
      if (nums1[i - 1] === nums2[j - 1]) {
        dp[j] = 1 + dpPrev[j - 1]
      } else {
        dp[j] = Math.max(dp[j - 1], dpPrev[j])
      }
    }

    dpPrev = dp.slice()
  }

  return dp[len2]
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 4, 2], [1, 2, 4]],
    [[2, 5, 1, 2, 5], [10, 5, 2, 1, 5, 2]],
    [[1, 3, 7, 1, 7, 5], [1, 9, 2, 5, 1]],
    [[3, 2], [2, 2, 2, 3]]
  ]

  for (const [nums1, nums2] of inputs) {
    const result = maxUncrossedLines(nums1, nums2)
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
