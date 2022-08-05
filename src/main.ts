function combinationSum4 (nums: number[], target: number): number {
  const result = new Array<number>(target + 1).fill(0)
  result[0] = 1
  for (let i = 1; i <= target; i += 1) {
    for (const num of nums) {
      if (i >= num) {
        result[i] += result[i - num]
      }
    }
  }

  return result[target]
}

interface Input {
  readonly nums: number[]
  readonly target: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3],
      target: 4
    },
    {
      nums: [9],
      target: 3
    }
  ]

  for (const { nums, target } of inputs) {
    const result = combinationSum4(nums, target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
