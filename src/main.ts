/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes (nums: number[]): void {
  let lastZero = 0

  for (let i = 0; i < nums.length; i += 1) {
    if (nums[i] !== 0) {
      nums[lastZero] = nums[i]
      lastZero += 1
    }
  }

  for (let i = lastZero; i < nums.length; i += 1) {
    nums[i] = 0
  }
}

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [0, 1, 0, 3, 12]
    },
    {
      nums: [0]
    }
  ]

  for (const { nums } of inputs) {
    moveZeroes(nums)
    console.log(nums)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
