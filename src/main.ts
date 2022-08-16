function majorityElement (nums: number[]): number {
  let count = 0
  let candidate = nums[0]

  for (const num of nums) {
    if (count < 1) {
      candidate = num
    }

    count += (num === candidate) ? 1 : -1
  }

  return candidate
}

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [3, 2, 3]
    },
    {
      nums: [2, 2, 1, 1, 1, 2, 2]
    }
  ]

  for (const { nums } of inputs) {
    const result = majorityElement(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
