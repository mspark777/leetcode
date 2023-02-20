function searchInsert (nums: number[], target: number): number {
  if (target < (nums.at(0) as number)) {
    return 0
  } else if (target > (nums.at(-1) as number)) {
    return nums.length
  }

  let begin = 0n
  let end = BigInt(nums.length)
  while (begin < end) {
    const middle = (begin + end) / 2n
    const pos = Number(middle)
    const num = nums[pos]
    if (num < target) {
      begin = middle + 1n
    } else if (num > target) {
      end = middle
    } else {
      return pos
    }
  }

  return Number(begin)
}

interface Input {
  readonly nums: number[]
  readonly target: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { nums: [1, 3, 5, 6], target: 5 },
    { nums: [1, 3, 5, 6], target: 2 },
    { nums: [1, 3, 5, 6], target: 7 }
  ]

  for (const { nums, target } of inputs) {
    const result = searchInsert(nums, target)
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
