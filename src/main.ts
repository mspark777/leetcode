function largestPerimeter (nums: number[]): number {
  nums.sort((a, b) => b - a)
  for (let i = 0; i <= nums.length - 3; i += 1) {
    const a = nums[i]
    const b = nums[i + 1] + nums[i + 2]
    if (a < b) {
      return a + b
    }
  }

  return 0
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [2, 1, 2],
    [1, 2, 1]
  ]

  for (const nums of inputs) {
    const result = largestPerimeter(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
